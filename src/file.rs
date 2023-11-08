use std::{fs, io::{BufReader, Read}};

pub fn load_file(path: String) -> Result<(Vec<u8>, Option<String>), String> {

    let mut path = path;
    
    if path.ends_with("/") {//load index.html if path is a directory
        path = path + "index.html";
    }

    let file = match fs::File::open(&path) {
        Ok(f) => f,
        Err(err) => return Err(err.to_string()),
    };
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::<u8>::new();

    match reader.read_to_end(&mut buffer) {
        Ok(count) => count,
        Err(err) => return Err(err.to_string()),
    };

    return Ok((buffer, content_type(&path)));
}

fn content_type(path: &String) -> Option<String> {
    let lower = path.to_lowercase();
    let splitted = lower.split(".");
    let file_extension = match splitted.last() {
        Some(e) => e,
        None => return None
    };
    match file_extension {
        "html" => Some(String::from("text/html")),
        "css" => Some(String::from("text/css")),
        "js" => Some(String::from("text/javascript")),
        "txt" => Some(String::from("text/plain")),
        "xml" => Some(String::from("application/xml")),
        "json" => Some(String::from("application/json")),
        "pdf" => Some(String::from("application/pdf")),
        "zip" => Some(String::from("application/zip")),
        "jpg" | "jpeg" | "jfif" | "pjpeg" | "pjp" => Some(String::from("image/jpeg")),
        "png" => Some(String::from("image/png")),
        "svg" => Some(String::from("image/svg+xml")),
        "ico" => Some(String::from("image/vnd.microsoft.icon")),
         _ => None,
    }
}
