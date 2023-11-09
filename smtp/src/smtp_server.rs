use smtp::core::{SmtpSessionManager, SMTP};
use utils::{
    config::{Config, ServerProtocol},
    enable_tracing, UnwrapFailure,
};
use jmap::services::IPC_CHANNEL_BUFFER;
use directory::config::ConfigDirectory;
use tokio::sync::mpsc;

#[no_mangle]
pub async fn start_smtp_server () {
    let config = Config::init();
    let servers = config.parse_servers().failed("Invalid configuration");
    let directory = config.parse_directory().failed("Invalid configuration");

    servers.bind(&config);

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
    let smtp = SMTP::init(&config, &servers, &directory, delivery_tx)
        .await
        .failed("Invalid configuration file");

        let (shutdown_tx, shutdown_rx) = servers.spawn(|server, shutdown_rx| {
            match &server.protocol {
                ServerProtocol::Smtp | ServerProtocol::Lmtp => {
                    server.spawn(SmtpSessionManager::new(smtp.clone()), shutdown_rx)
                }
                _ => todo!()
            }
        });
}
