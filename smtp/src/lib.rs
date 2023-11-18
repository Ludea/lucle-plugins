mod smtp_server;

#[no_mangle]
pub async fn run() {
    smtp_server::start_smtp_server().await;
}
