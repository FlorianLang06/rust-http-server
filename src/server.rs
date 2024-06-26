use crate::config::Config;
use crate::file;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpListener;

pub async fn listen_tcp(config: Config) {
    let addr = format!("{}:{}", config.server_ip(), config.port());
    let listener = match TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(err) => {
            println!("Failed to bind on {} Message: {}", &addr, err);
            return;
        }
    };

    println!("Listening TCP on {}", &addr);

    loop {
        let stream = match listener.accept().await {
            Err(err) => {
                println!("Failed to accept connection: {}", err);
                continue;
            }
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
                let mut split = lines[0].split(" ");
                let method = match split.next() {
                    None => return,
                    Some(m) => m.to_string(),
                };
                let path = match split.next() {
                    None => return,
                    Some(p) => p.to_string(),
                };
                let version = match split.next() {
                    None => return,
                    Some(v) => v.trim().to_string(),
                };

                let mut headers = Vec::<Header>::new();
                for i in 1..lines.len() {
                    let mut  split = lines[i].split(": ");

                    let key = match split.next() {
                        Some(k) => k.to_string(),
                        None => continue,
                    };

                    let value = match split.next() {
                        Some(v) => v.to_string(),
                        None => continue,
                    };

                    headers.push(Header::new(key, value));
                }

                let request = HttpRequest::new(method, path, version, headers);
                let response = handle_request(request, &config);
                write_response(response, &mut writer).await;

                //let content = "<html><body><b>test</b></body></html>\r\n".as_bytes();

                break;
            }
            lines.push(line);
            //print!("{}", line);
        }
    }
}

fn handle_request(request: HttpRequest, config: &Config) -> HttpResponse {
    if request.method != String::from("GET") {
        return HttpResponse::method_not_allowed(request.version);
    }

    let phy_path = match physical_path(&request, config) {
        Some(p) => p,
        None => return HttpResponse::not_found(request.version),
    };

    let (file_content, content_type) = match file::load_file(phy_path) {
        Ok(c) => c,
        Err(err) => {
            println!("{}", err);
            return HttpResponse::not_found(request.version);
        }
    };
    HttpResponse::ok(
        request.version,
        Some(file_content),
        content_type,
    )
}

fn physical_path(request: &HttpRequest, config: &Config) -> Option<String> {
    let sites = config.sites();
    for site in sites {
        let hostname = site.hostname();
        if hostname.trim() == "*" || (request.host().is_some() && hostname.trim() == request.host().unwrap().trim()) {
            let phy_path = match site.physical_path() {
                Some(p) => p,
                None => return None,
            };
            return Some(phy_path + &request.path);
        }
    }
    return None;
}

async fn write_response(response: HttpResponse, writer: &mut BufWriter<OwnedWriteHalf>) {
    let _ = writer
        .write(
            format!(
                "{} {} {}\r\n",
                response.version, response.status, response.status_message
            )
            .as_bytes(),
        )
        .await;
    if response.body.is_some() {
        let body = match response.body {
            None => return,
            Some(b) => b,
        };
        let _ = writer
            .write(format!("Content-Length: {}\r\n", body.len()).as_bytes())
            .await;
        let content_type = match response.content_type {
            Some(c) => c,
            None => String::from("text/plain"),
        };

        let _ = writer.write(format!("Content-Type: {}\r\n", content_type).as_bytes()).await;
        let _ = writer.write("\r\n".as_bytes()).await;
        let _ = writer.write(body.as_slice()).await;
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
    headers: Vec<Header>
}

impl HttpRequest {
    pub fn new(method: String, path: String, version: String, headers: Vec<Header>) -> Self {
        Self {
            method,
            path,
            version,
            headers
        }
    }

    pub fn host(&self) -> Option<String> {
        let host = self.headers.iter().find(|x| x.key == "Host");
        match host {
            Some(h) => Some(h.value.clone()),
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    version: String,
    status: u16,
    status_message: String,
    body: Option<Vec<u8>>,
    content_type: Option<String>
}

impl HttpResponse {
    fn new(version: String, status: u16, status_message: String, body: Option<Vec<u8>>, content_type: Option<String>) -> Self {
        Self {
            version,
            status,
            status_message,
            body,
            content_type
        }
    }

    fn ok(version: String, body: Option<Vec<u8>>, content_type: Option<String>) -> Self {
        Self::new(version, 200, String::from("Ok"), body, content_type)
    }

    fn method_not_allowed(version: String) -> Self {
        Self::new(version, 405, String::from("Method not Allowed"), None, None)
    }

    fn not_found(version: String) -> Self {
        Self::new(version, 404, String::from("Not Found"), None, None)
    }
}

#[derive(Debug)]
pub struct Header {
    key: String,
    value: String
}


impl Header {
    fn new(key: String, value: String) -> Self {
        Self {
            key,
            value
        }
    }
}
