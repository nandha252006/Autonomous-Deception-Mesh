mod server;
mod logging;
use server::http::start_http;
use logging::logger::start_logger;
#[tokio::main]
async fn main() {
    start_logger();
    start_http().await;
}
