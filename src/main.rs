mod network;
mod request;

#[tokio::main]
async fn main() {

    network::listen_tcp("0.0.0.0:8080").await;
}
