mod server;
mod logging;
use server::http::start_http;
use logging::logger::start_logger;
use server::ftp::start_ftp;
#[tokio::main]
// This main get stuck at start_ftp() as the thread is blocked by await , So the next process that is start_http will nevet get started.
// async fn main() {
//     start_logger();
//     start_ftp().await;
//     start_http().await;
// }
/* So , To Execute several async fn inside a single fn we need to provide seperate threads for each processes using 
 * tokio::spawn(...) -> (Make this async execute independently) and
 * tokio::join!(...) -> (Aync fn gets dependent and wait until both the fn finshes or stopped)
 * */
async fn main(){
    start_logger();
    let start_ftp=tokio::spawn(async{
        start_ftp().await;
    });
    let start_http=tokio::spawn(async{
        start_http().await;
    });
    let _=tokio::join!(start_ftp,start_http);
}
