use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub async fn listen_tcp(addr: &str) {
    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(err) => {
            println!("Failed to bind on {} Message: {}", addr, err);
            return;
        }
    };

    println!("Listening TCP on {}", addr);

    loop {
        let mut stream = match listener.accept().await {
            Err(err) => {
                println!("Failed to accept connection: {}", err);
                continue
            },
            Ok((stream, _)) => stream,
        };

        match stream.write("test".as_bytes()).await {
            Ok(_) => {}
            Err(err) => {
                println!("Failed to write to stream: {}", err);
            }
        }
    }
}