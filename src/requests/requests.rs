use core::fmt;
use std::{collections::HashMap, io::Write};

#[derive(Debug, Default, PartialEq)]
pub enum RequestType {
    POST,
    PUT,
    DELETE,
    #[default]
    GET,
}

impl fmt::Display for RequestType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RequestType::PUT => write!(f, "PUT"),
            RequestType::GET => write!(f, "GET"),
            RequestType::POST => write!(f, "POST"),
            RequestType::DELETE => write!(f, "DELETE"),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Request {
    body: Option<String>,
    headers: HashMap<String, String>,
    path: String,
    host: String,
    request_type: RequestType,
}

impl Request {
    pub fn builder() -> RequestBuilder {
        RequestBuilder::new()
    }

    pub fn new(
        body: Option<String>,
        headers: HashMap<String, String>,
        path: String,
        host: String,
        request_type: RequestType,
    ) -> Self {
        Self {
            body,
            headers,
            path,
            host,
            request_type,
        }
    }

    fn headers_as_string(&self) -> String {
        let mut string_to_build = String::new();
        for (key, val) in &self.headers {
            string_to_build.push_str(&format!("{}: {}\r\n", key, val).to_string());
        }

        string_to_build
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();

        write!(
            &mut buffer,
            "{} {} HTTP/1.1\r\nHost: {}\r\n{}{}",
            self.request_type.to_string(),
            self.path,
            self.host,
            self.headers_as_string(),
            self.body.clone().unwrap_or("".to_string())
        )
        .unwrap();

        buffer
    }
}

#[derive(Debug, Default)]
pub struct RequestBuilder {
    body: Option<String>,
    headers: Option<HashMap<String, String>>,
    path: Option<String>,
    host: Option<String>,
    request_type: Option<RequestType>,
}

impl RequestBuilder {
    pub fn new() -> RequestBuilder {
        RequestBuilder {
            body: None,
            headers: None,
            path: None,
            host: None,
            request_type: None,
        }
    }

    pub fn with_body(mut self, body: String) -> RequestBuilder {
        self.body = Some(body);
        self
    }

    pub fn with_host(mut self, host: String) -> RequestBuilder {
        self.host = Some(host);
        self
    }

    pub fn with_headers(mut self, headers: HashMap<String, String>) -> RequestBuilder {
        self.headers = Some(headers);
        self
    }

    pub fn append_headers(mut self, headers: HashMap<String, String>) -> RequestBuilder {
        match self.headers {
            Some(_) => (),
            None => self.headers = Some(HashMap::new()),
        }

        let mut current = self.headers.unwrap();

        for (key, val) in headers {
            current.insert(key.to_string(), val.to_string());
        }
        self.headers = Some(current);
        self
    }

    pub fn with_path(mut self, path: String) -> RequestBuilder {
        self.path = Some(path);
        self
    }

    pub fn with_request_type(mut self, request_type: RequestType) -> RequestBuilder {
        self.request_type = Some(request_type);
        self
    }

    pub fn build(self) -> Result<Request, String> {
        Ok(Request::new(
            self.body,
            self.headers
                .ok_or("Make sure you input your headers.".to_string())?,
            self.path.ok_or("Each request needs a path.".to_string())?,
            self.host.ok_or("Each request needs a host.".to_string())?,
            self.request_type
                .ok_or("Request types are mandatory: GET, PUT, POST, DELETE".to_string())?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_builder_test_passes() {
        let req = Request {
            body: Some(String::from("some_bod")),
            headers: HashMap::from([("test".to_string(), "me".to_string())]),
            path: String::from("test_me_path"),
            host: String::from("Some-Host"),
            request_type: RequestType::GET,
        };

        let from_builder: Request = Request::builder()
            .with_body(String::from("some_bod"))
            .with_path(String::from("test_me_path"))
            .with_host(String::from("Some-Host"))
            .with_headers(HashMap::from([("test".to_string(), "me".to_string())]))
            .with_request_type(RequestType::GET)
            .build()
            .expect("Failed");

        assert_eq!(req, from_builder)
    }

    #[test]
    fn missing_request_type_fails() {
        let from_builder_with_no_req_type = Request::builder()
            .with_body("".to_string())
            .with_headers(HashMap::new())
            .with_host("".to_string())
            .with_path("".to_string())
            .build();

        assert!(from_builder_with_no_req_type.is_err());
    }

    #[test]
    fn missing_path_fails() {
        let from_builder_with_no_path = Request::builder()
            .with_headers(HashMap::from([("test".to_string(), "me".to_string())]))
            .with_request_type(RequestType::GET)
            .build();

        assert!(from_builder_with_no_path.is_err());
    }

    #[test]
    fn missing_headers_fails() {
        let from_builder_no_headers = Request::builder()
            .with_request_type(RequestType::GET)
            .with_path("".to_string())
            .build();

        assert!(from_builder_no_headers.is_err());
    }

    #[test]
    fn missing_body_passes() {
        let from_builder_no_body = Request::builder()
            .with_path("some_path".to_string())
            .with_headers(HashMap::from([("test".to_string(), "me".to_string())]))
            .with_request_type(RequestType::GET)
            .with_host("".to_string())
            .build();

        assert!(!from_builder_no_body.is_err());
    }
}
