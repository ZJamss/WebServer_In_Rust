use std::collections::HashMap;
use regex::Regex;
use crate::http_request::Resource::Path;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

impl From<&str> for Resource {
    fn from(s: &str) -> Self {
        Path(s.to_string())
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";
        let top_line_regex = Regex::new(".* .* HTTP/").unwrap();
        let header_line_regex = Regex::new(".*:\\s.*").unwrap();
        for line in req.lines() {
            if top_line_regex.is_match(line) {
                let (method, resource, version) = process_top_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if header_line_regex.is_match(line) {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else if line.len() == 0 {} else {
                parsed_msg_body = line;
            }
        };

        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn process_top_line(line: &str) -> (Method, Resource, Version) {
    let mut words = line.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        resource.into(),
        version.into()
    )
}

fn process_header_line(line: &str) -> (String, String) {
    let mut header_item = line.split(":");

    let mut key = String::from("");
    let mut value = String::from("");
    if let Some(k) = header_item.next() {
        key = k.trim().to_string();
    }
    if let Some(v) = header_item.next() {
        value = v.trim().to_string();
    }

    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(Method::Get, m);
    }

    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }

    #[test]
    fn test_read_http() {
        let request: HttpRequest =
"GET / HTTP/1.1
Host: localhost:3000
Connection: keep-alive
Cache-Control: max-age=0
sec-ch-ua-mobile: ?0
Upgrade-Insecure-Requests: 1
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9
Sec-Fetch-Site: cross-site
Sec-Fetch-Mode: navigate
Sec-Fetch-User: ?1
Sec-Fetch-Dest: document
Accept-Encoding: gzip, deflate, br
Accept-Language: zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7,zh-TW;q=0.6
Cookie: ts_uid=2969209092".to_string().into();
        assert_eq!(request.method, Method::Get);
        assert_eq!(request.headers.get("Sec-Fetch-Dest").unwrap(), "document");
    }
}