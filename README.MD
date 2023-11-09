# Simple Rust HTTP Server 

This is a simple project where I tried to write an http server in rust. I am not a rust expert, so the code and the code stucture could be bad. 

The server can only handle http GET request and respone with the content of the file in the filesystem.

If you request a path the server try to read the `index.html` file in this path.

## Configuration 
The config is stored in the `http-server.json` file in the root directory of the server.

### Example config

```json
{
  "server_ip": "0.0.0.0",
  "port": 8080,
  "sites": [
    {
      "hostname": "*",
      "physical_path": "./testsite"
    }
  ]
}
```
