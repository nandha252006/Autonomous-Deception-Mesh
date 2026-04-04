use tracing_subscriber::{fmt,EnvFilter};
use tracing::subscriber::set_global_default;
use std::fs::OpenOptions;

pub fn start_logger(){
    // OpenOptions - Open files with configurable chance --> provideed with a chain og options like create , write , append before opening the file
    // OpenOptions - return type => Result<std::fs::file,std::io::Error>
    let file=OpenOptions::new()
        .create(true)
        .append(true)
        .open("src/logging/honeypot.log")
        .expect("Failed to open the log file");
    // fmt() => Used to format the logs
    let subscriber=fmt()
        // .with_env_filter(EnvFilter::from_default_env())
        .with_max_level(tracing::Level::INFO)
        .with_writer(move || file.try_clone().unwrap())
        .finish();
    // Used to pass all the logs through the subscribers
    set_global_default(subscriber)
        .expect("Failed to set logger");
}
