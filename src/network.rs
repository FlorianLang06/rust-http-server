use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::tcp::OwnedWriteHalf;
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

        let mut lines = Vec::<String>::new();
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
                print!("{}", lines[0]);
                let mut split = lines[0].split(" ");
                let method = match split.next() {
                    None => return,
                    Some(m) => m.to_string(),
                };
                let path = match split.next() {
                    None => return,
                    Some(p) => p.to_string()
                };
                let version = match split.next() {
                    None => return,
                    Some(v) => v.trim().to_string(),
                };

                let request = HttpRequest::new(method, path, version);
                let response = handle_request(request);
                write_response(response, &mut writer).await;

                //let content = "<html><body><b>test</b></body></html>\r\n".as_bytes();



                break;
            }
            lines.push(line);
            //print!("{}", line);
        }
    }
}

fn handle_request(request: HttpRequest) -> HttpResponse {
    if request.method != String::from("GET") {
        return HttpResponse::method_not_allowed(request.version);
    }

    if request.path == String::from("/") {
        return HttpResponse::ok(request.version, Some(String::from("<html><body><b>test</b></body></html>")));
    }

    HttpResponse::not_found(request.version)
}

async fn write_response(response: HttpResponse, writer: &mut BufWriter<OwnedWriteHalf>) {
    let _ = writer.write(format!("{} {} {}\r\n", response.version, response.status, response.status_message).as_bytes()).await;
    if response.body.is_some() {
        let body = match response.body {
            None => return,
            Some(b) => b,
        };
        let _ = writer.write(format!("Content-Length: {}\r\n", body.len()).as_bytes()).await;
        let _ = writer.write("Content-Type: text/html\r\n".as_bytes()).await;
        let _ = writer.write("\r\n".as_bytes()).await;
        let _ = writer.write(body.as_bytes()).await;
    } else {
        let _ = writer.write("\r\n".as_bytes()).await;
    }

    let _ = writer.flush().await;
}

#[derive(Debug)]
pub struct HttpRequest {
    method: String,
    path: String,
    version: String,
}

impl HttpRequest {
    pub fn new(method: String, path: String, version: String) -> Self {
        Self {
            method,
            path,
            version
        }
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    version: String,
    status: u16,
    status_message: String,
    body: Option<String>,
}

impl HttpResponse {
    fn new(version: String, status: u16, status_message: String, body: Option<String>) -> Self {
        Self {
            version,
            status,
            status_message,
            body
        }
    }

    fn ok(version: String, body: Option<String>) -> Self {
        Self::new(version, 200, String::from("Ok"), body)
    }

    fn method_not_allowed(version: String) -> Self {
        Self::new(version, 405, String::from("Method not Allowed"), None)
    }

    fn not_found(version: String) -> Self {
        Self::new(version, 404, String::from("Not Found"), None)
    }
}