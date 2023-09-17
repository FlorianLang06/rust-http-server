mod network;

#[tokio::main]
async fn main() {

    network::listen_tcp("0.0.0.0:8080").await;
}
