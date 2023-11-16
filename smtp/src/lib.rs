use tokio;

mod smtp_server;

#[no_mangle]
pub fn run() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let join_handle = rt.spawn(async {
        smtp_server::start_smtp_server().await;
      });
      rt.block_on(join_handle).unwrap();
}
