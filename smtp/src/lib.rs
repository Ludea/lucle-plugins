pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod smtp_server;

pub async fn start_smtp_server() {
    smtp_server::start_smtp_server().await;
}