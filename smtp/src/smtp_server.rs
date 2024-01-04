use ahash::{AHashMap, AHashSet};
use directory::core::config::ConfigDirectory;
use jmap::services::IPC_CHANNEL_BUFFER;
use smtp::core::{SmtpSessionManager, SMTP};
use store::config::ConfigStore;
use std::collections::BTreeMap;
use tokio::sync::mpsc;
use utils::{
    config::{Config, ServerProtocol},
    enable_tracing, failed, UnwrapFailure,
};

pub async fn start_smtp_server() {
    let mut config = Config::default();
    config
        .parse(&std::fs::read_to_string("config.toml").unwrap())
        .failed("Could not read configuration file");

    // Extract macros and includes
    let mut keys = BTreeMap::new();
    let mut includes = AHashSet::new();
    let mut macros = AHashMap::new();
    for (key, value) in config.keys {
        if let Some(macro_name) = key.strip_prefix("macros.") {
            macros.insert(macro_name.to_ascii_lowercase(), value);
        } else if key.starts_with("include.files.") {
            includes.insert(value);
        } else {
            keys.insert(key, value);
        }
    }

    // Include files
    config.keys = keys;
    for mut include in includes {
        include.replace_macros("include.files", &macros);
        config
            .parse(&std::fs::read_to_string(&include).failed(&format!(
                "Could not read included configuration file {include:?}"
            )))
            .failed(&format!("Invalid included configuration file {include:?}"));
    }

    // Replace macros
    for (key, value) in &mut config.keys {
        value.replace_macros(key, &macros);
    }

    let servers = config.parse_servers().failed("Invalid configuration");
    servers.bind(&config);

     // Parse stores and directories
    let stores = config.parse_stores().await.failed("Invalid configuration");
    let directory = config
        .parse_directory(&stores, config.value("jmap.store.data"))
        .await
        .failed("Invalid configuration");
    let schedulers = config
        .parse_purge_schedules(
            &stores,
            config.value("jmap.store.data"),
            config.value("jmap.store.blob"),
        )
        .await
        .failed("Invalid configuration");

    // Enable tracing
    let _tracer = enable_tracing(
        &config,
        &format!(
            "Starting Stalwart Mail Server v{}...",
            env!("CARGO_PKG_VERSION"),
        ),
    )
    .failed("Failed to enable tracing");

    let (delivery_tx, delivery_rx) = mpsc::channel(IPC_CHANNEL_BUFFER);
    let smtp = SMTP::init(&config, &servers, &store, &directory, delivery_tx)
        .await
        .failed("Invalid configuration file");

    let (shutdown_tx, shutdown_rx) = servers.spawn(|server, shutdown_rx| match &server.protocol {
        ServerProtocol::Smtp | ServerProtocol::Lmtp => {
            server.spawn(SmtpSessionManager::new(smtp.clone()), shutdown_rx)
        }
        _ => todo!(),
    });

        // Spawn purge schedulers
    for scheduler in schedulers {
        scheduler.spawn(shutdown_rx.clone());
    }

    // Wait for shutdown signal
    wait_for_shutdown(&format!(
        "Shutting down Stalwart Mail Server v{}...",
        env!("CARGO_PKG_VERSION")
    ))
    .await;

    // Stop services
    let _ = shutdown_tx.send(true);

    // Wait for services to finish
    tokio::time::sleep(Duration::from_secs(1)).await;
}

trait ReplaceMacros: Sized {
    fn replace_macros(&mut self, key: &str, macros: &AHashMap<String, String>);
}

impl ReplaceMacros for String {
    fn replace_macros(&mut self, key: &str, macros: &AHashMap<String, String>) {
        if self.contains("%{") {
            let mut result = String::with_capacity(self.len());
            let mut value = self.as_str();

            loop {
                if let Some((suffix, macro_name)) = value.split_once("%{") {
                    if !suffix.is_empty() {
                        result.push_str(suffix);
                    }
                    if let Some((macro_name, rest)) = macro_name.split_once("}%") {
                        if let Some(macro_value) = macros.get(&macro_name.to_ascii_lowercase()) {
                            result.push_str(macro_value);
                            value = rest;
                        } else {
                            failed(&format!("Unknown macro {macro_name:?} for key {key:?}"));
                        }
                    } else {
                        failed(&format!(
                            "Unterminated macro name {value:?} for key {key:?}"
                        ));
                    }
                } else {
                    result.push_str(value);
                    break;
                }
            }

            *self = result;
        }
    }
}
