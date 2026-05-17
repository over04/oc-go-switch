mod business;
mod common;
mod init;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(false)
        .init();

    init::runtime::run().await;
}
