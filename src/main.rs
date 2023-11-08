mod server;
mod config;
mod file;

#[tokio::main]
async fn main() {

    let config = config::load_config();

    server::listen_tcp(config).await;
}
