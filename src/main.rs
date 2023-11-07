mod network;
mod config;
mod file;

#[tokio::main]
async fn main() {

    let config = config::load_config();

    network::listen_tcp(config).await;
}
