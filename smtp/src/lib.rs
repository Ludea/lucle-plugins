use tokio;

mod smtp_server;

#[no_mangle]
pub fn run() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(smtp_server::start_smtp_server());
}
