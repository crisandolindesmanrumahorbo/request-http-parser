//! Parse string request from incoming http to Request struct model which include method, path,
//! headers, params (optional) and body (optional)
use anyhow::{Context, Result, anyhow};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    GET,
    POST,
    OPTIONS,
}

impl TryFrom<&str> for Method {
    type Error = anyhow::Error;
    fn try_from(value: &str) -> Result<Self, anyhow::Error> {
        match value {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            "OPTIONS" => Ok(Method::OPTIONS),
            _ => Err(anyhow!("Method not supported")),
        }
    }
}

pub struct Request {
    pub method: Method,
    pub path: String,
    pub params: Option<std::collections::HashMap<String, String>>,
    pub headers: std::collections::HashMap<String, String>,
    pub body: Option<String>,
}

impl Request {
    /// # Examples
    ///
    /// ```
    /// use request_http_parser::parser::{Method,Request};
    ///     let req_str = format!(
    ///         "POST /login HTTP/1.1\r\n\
    ///         Content-Type: application/json\r\n\
    ///         User-Agent: Test\r\n\
    ///         Content-Length: {}\r\n\\r\n\
    ///         {{\"username\": \"{}\",\"password\": \"{}\"}}",
    ///         44, "crisandolin", "rumahorbo");
    ///     let req = Request::new(&req_str).unwrap();
    ///
    ///     assert_eq!(Method::POST, req.method);
    ///     assert_eq!("/login", req.path);  
    /// ```
    ///
    pub fn new(request: &str) -> Result<Self> {
        let mut parts = request.split("\r\n\r\n");
        let head = parts.next().context("Headline Error")?;
        // Body
        let body = parts.next().map(|b| b.to_string());

        // Method and path
        let mut head_line = head.lines();
        let first: &str = head_line.next().context("Empty Request")?;
        let mut request_parts: std::str::SplitWhitespace<'_> = first.split_whitespace();
        let method: Method = request_parts
            .next()
            .ok_or(anyhow!("missing method"))
            .and_then(TryInto::try_into)
            .context("Missing Method")?;
        let url = request_parts.next().context("No Path")?;
        let (path, params) = Self::extract_query_param(url);

        // Headers
        let mut headers = HashMap::new();
        for line in head_line {
            if let Some((k, v)) = line.split_once(":") {
                headers.insert(k.trim().to_lowercase(), v.trim().to_string());
            }
        }
        Ok(Request {
            method,
            path,
            headers,
            body,
            params,
        })
    }

    /// extract query param from url
    fn extract_query_param(url: &str) -> (String, Option<HashMap<String, String>>) {
        // Find the query string
        if let Some(pos) = url.find('?') {
            let path = &url[0..pos];
            let query_string = &url[pos + 1..]; // Get substring after '?'

            // Parse query params into a HashMap
            let params: HashMap<_, _> = query_string
                .split('&')
                .filter_map(|pair| {
                    let mut kv = pair.split('=');
                    Some((kv.next()?.to_string(), kv.next()?.to_string()))
                })
                .collect();

            // Return the token if it exists
            (path.to_string(), Some(params))
            // params.get("token").map(|s| s.to_string())
        } else {
            (url.to_string(), None)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::Method;

    use super::Request;

    #[test]
    fn parser() {
        let req_str = format!(
            "POST /login HTTP/1.1\r\n\
                Content-Type: application/json\r\n\
                User-Agent: Test\r\n\
                Content-Length: {}\r\n\
                \r\n\
                {{\"username\": \"{}\",\"password\": \"{}\"}}",
            44, "crisandolin", "rumahorbo"
        );

        let req = Request::new(&req_str).unwrap();

        assert_eq!(Method::POST, req.method);
        assert_eq!("/login", req.path);
    }
}
