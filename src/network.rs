use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
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
        let stream = match listener.accept().await {
            Err(err) => {
                println!("Failed to accept connection: {}", err);
                continue
            },
            Ok((stream, _)) => stream,
        };

        let (read_half, write_half) = stream.into_split();

        let mut reader = BufReader::new(read_half);
        let mut writer = BufWriter::new(write_half);

        loop {
            let mut line = String::new();

            let count = match reader.read_line(&mut line).await {
                Ok(c) => c,
                Err(_) => {
                    break;
                }
            };
            if count < 1 {
                break;
            }
            if line == "\r\n" {
                let content = "<html><body>test</body></html>\r\n".as_bytes();
                let _ = writer.write("HTTP/1.1 200 OK\r\n".as_bytes()).await;
                let _ = writer.write(format!("Content-Length: {}\r\n", content.len()).as_bytes()).await;
                let _ = writer.write("Content-Type: text/html\r\n".as_bytes()).await;
                let _ = writer.write("\r\n".as_bytes()).await;
                let _ = writer.write(content).await;
                let _ = writer.flush().await;
                break;
            }

            print!("{}", line);
        }
    }
}