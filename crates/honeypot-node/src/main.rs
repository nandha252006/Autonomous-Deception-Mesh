mod server;
use server::http::start_http;
#[tokio::main]
async fn main() {
    start_http().await;
}
