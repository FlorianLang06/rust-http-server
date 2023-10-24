#[derive(Debug)]
pub struct Request {
    method: String,
    path: String,
    version: String,
}

impl Request {
    pub fn new(method: String, path: String, version: String) -> Self {
        Self {
            method,
            path,
            version
        }
    }
}