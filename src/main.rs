use log::LevelFilter;
use crate::logger::SimpleLogger;

mod server;
mod config;
mod file;
mod logger;

static LOGGER: SimpleLogger = SimpleLogger;

#[tokio::main]
async fn main() {
    let config = config::load_config();

    let _ = log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info));

    server::listen_tcp(config).await;
}
