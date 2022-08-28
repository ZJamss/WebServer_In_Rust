use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_msg: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_msg: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(_res: HttpResponse) -> String {
        let res = _res.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &res.version(),
            &res.status_code(),
            &res.status_msg(),
            &res.headers(),
            &_res.body.unwrap().len(),
            &res.body()
        )
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(status_code: &'a str, headers: Option<HashMap<&'a str, &'a str>>, body: Option<String>) -> Self {
        let mut response: HttpResponse<'a> = HttpResponse::default();
        if status_code != "200" {
            response.status_code = status_code.into();
        };

        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            }
        };

        response.status_msg = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "404" => "Not Found".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(),
        };

        response.body = body;

        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<(), ()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);

        Ok(())
    }

    pub fn version(&self) -> &'a str {
        self.version
    }
    pub fn status_code(&self) -> &'a str {
        self.status_code
    }
    pub fn status_msg(&self) -> &'a str {
        self.status_msg
    }
    pub fn headers(&self) -> String {
        let headers = self.headers.as_ref().unwrap();
        let mut header_string: String = "".into();
        for (k, v) in headers {
            header_string = format!("{}{}: {}\r\n", header_string, k, v);
        }
        header_string
    }
    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => ""
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new(
            "200",
            None,
            Some("xxx".into()),
        );

        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "200",
            status_msg: "OK",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxx".into()),
        };

        assert_eq!(response_expected, response_actual);
    }

    #[test]
    fn test_response_struct_creation_404() {
        let response_actual = HttpResponse::new(
            "404",
            None,
            Some("xxx".into()),
        );

        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_msg: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxx".into()),
        };

        assert_eq!(response_expected, response_actual);
    }

    #[test]
    fn test_response_struct_creation() {
        let response_expected = HttpResponse {
            version: "HTTP/1.1",
            status_code: "404",
            status_msg: "Not Found",
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type", "text/html");
                Some(h)
            },
            body: Some("xxx".into()),
        };
        let http_string:String  = response_expected.into();
        let actual_string =
        "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: 3\r\n\r\nxxx";
        assert_eq!(actual_string, http_string);
    }
}

