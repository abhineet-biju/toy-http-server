use std::io::Write;

#[derive(Debug, Clone)]
pub struct Response {
    pub status_code: u16,
    pub status_text: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

impl Response {
    pub fn new(status_code: u16) -> Self {
        Self {
            status_code,
            status_text: status_text(status_code).to_string(),
            headers: Vec::new(),
            body: None,
        }
    }

    pub fn ok() -> Self {
        Self::new(200)
    }

    pub fn created() -> Self {
        Self::new(201)
    }

    pub fn no_content() -> Self {
        Self::new(204)
    }

    pub fn bad_request() -> Self {
        Self::new(400)
    }

    pub fn not_found() -> Self {
        Self::new(404)
    }

    pub fn method_not_allowed() -> Self {
        Self::new(405)
    }

    pub fn internal_error() -> Self {
        Self::new(500)
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }

    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        let body_in_bytes = body.into();

        self.headers.push((
            "Content-Length".to_string(),
            body_in_bytes.len().to_string(),
        ));

        self.body = Some(body_in_bytes);
        self
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text,);

        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }

        response.push_str("\r\n");

        let mut bytes = response.into_bytes();

        if let Some(body) = &self.body {
            bytes.extend_from_slice(body);
        }

        bytes
    }

    pub fn send(&self, stream: &mut impl Write) -> std::io::Result<()> {
        stream.write_all(&self.to_bytes())?;
        stream.flush()?;

        Ok(())
    }
}
fn status_text(status_code: u16) -> &'static str {
    match status_code {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        301 => "Moved Permanently",
        302 => "Found",
        304 => "Not Modified",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        413 => "Payload Too Large",
        500 => "Internal Server Error",
        501 => "Not Implemented",
        503 => "Service Unavailable",
        _ => "Unknown",
    }
}
