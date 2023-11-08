use std::fs;

pub fn load_file(path: String) -> Result<String, String> {

    let mut path = path;
    
    if path.ends_with("/") {//load index.html if path is a directory
        path = path + "index.html";
    }

    let file_content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(err) => return Err(err.to_string()),
    };

    return Ok(file_content);
}