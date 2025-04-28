# How to use

```
let mut buffer = [0; 1024];
let size =  stream.read(&mut buffer).expect("");
let req_str = String::from_utf8_lossy(&buffer[..size]);
let req = Request::new(&req_str);

```

# Examples

```
    use request_http_parser::parser::{Method,Request};
    let req_str = format!(
                "POST /login HTTP/1.1\r\n\
                Content-Type: application/json\r\n\
                User-Agent: Test\r\n\
                Content-Length: {}\r\n\\r\n\
                {{\"username\": \"{}\",\"password\": \"{}\"}}",
                44, "crisandolin", "rumahorbo");
                let req = Request::new(&req_str);

    assert_eq!(Method::POST, req.method);
    assert_eq!("/login", req.path);
```
