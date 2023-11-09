pub fn add(left: usize, right: usize) -> usize {
    left + right
}

mod smtp_server;

pub async fn run() {
    smtp_server::start_smtp_server().await;
}