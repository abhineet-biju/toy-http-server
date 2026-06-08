#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub query: Option<String>,
    pub version: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidRequestLine,
    InvalidMethod,
    InvalidHeader,
    IncompleteRequest,
    InvalidUtf8,
}

impl Request {
    pub fn parse(raw: &[u8]) -> Result<Self, ParseError> {
        let request_str = std::str::from_utf8(raw).map_err(|_| ParseError::InvalidUtf8)?;

        let mut parts = request_str.splitn(2, "\r\n\r\n");
        let header_section = parts.next().ok_or(ParseError::IncompleteRequest)?;
        let body_section = parts.next().ok_or(ParseError::IncompleteRequest)?;

        let mut lines = header_section.lines();

        let request_line = lines.next().ok_or(ParseError::InvalidRequestLine)?;
        let mut request_line_parts = request_line.split_whitespace();

        let method_str = request_line_parts
            .next()
            .ok_or(ParseError::InvalidRequestLine)?;
        let full_path = request_line_parts
            .next()
            .ok_or(ParseError::InvalidRequestLine)?;
        let version = request_line_parts
            .next()
            .ok_or(ParseError::InvalidRequestLine)?;

        if request_line_parts.next().is_some() {
            return Err(ParseError::InvalidRequestLine);
        }

        let method = match method_str {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "PATCH" => Method::PATCH,
            "HEAD" => Method::HEAD,
            "OPTIONS" => Method::OPTIONS,
            _ => return Err(ParseError::InvalidMethod),
        };

        let (path, query) = match full_path.split_once('?') {
            Some((path, query)) => (path.to_string(), Some(query.to_string())),
            None => (full_path.to_string(), None),
        };

        let mut headers = Vec::new();

        for line in lines {
            if line.is_empty() {
                break;
            }

            let (key, value) = line.split_once(":").ok_or(ParseError::InvalidHeader)?;

            if key.trim().is_empty() {
                return Err(ParseError::InvalidHeader);
            }

            headers.push((key.trim().to_string(), value.trim().to_string()));
        }

        let body = if body_section.is_empty() {
            None
        } else {
            Some(body_section.to_string())
        };

        Ok(Self {
            method,
            path,
            query,
            version: version.to_string(),
            headers,
            body,
        })
    }

    pub fn get_header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(key, _)| key.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_get_request() {
        let raw = b"GET /hello?name=abhineet HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl\r\n\r\n";

        let request = Request::parse(raw).unwrap();

        assert_eq!(request.method, Method::GET);
        assert_eq!(request.path, "/hello");
        assert_eq!(request.query, Some("name=abhineet".to_string()));
        assert_eq!(request.version, "HTTP/1.1");
        assert_eq!(request.get_header("host"), Some("localhost:8080"));
        assert_eq!(request.body, None);
    }

    #[test]
    fn parse_post_request_with_body() {
        let raw =
            b"POST /echo HTTP/1.1\r\nHost: localhost:8080\r\nContent-Length: 11\r\n\r\nhello=world";

        let request = Request::parse(raw).unwrap();

        assert_eq!(request.method, Method::POST);
        assert_eq!(request.path, "/echo");
        assert_eq!(request.query, None);
        assert_eq!(request.get_header("Content-Length"), Some("11"));
        assert_eq!(request.body, Some("hello=world".to_string()));
    }

    #[test]
    fn reject_invalid_method() {
        let raw = b"INVALID / HTTP/1.1\r\nHost: localhost:8080\r\n\r\n";

        let error = Request::parse(raw).unwrap_err();

        assert_eq!(error, ParseError::InvalidMethod);
    }

    #[test]
    fn reject_invalid_request_line() {
        let raw = b"GET / HTTP/1.1 invalid\r\nHost: localhost:8080\r\n\r\n";

        let error = Request::parse(raw).unwrap_err();

        assert_eq!(error, ParseError::InvalidRequestLine);
    }

    #[test]
    fn reject_invalid_header() {
        let raw = b"GET / HTTP/1.1\r\nInvalid Header\r\n\r\n";

        let error = Request::parse(raw).unwrap_err();

        assert_eq!(error, ParseError::InvalidHeader);
    }
}
