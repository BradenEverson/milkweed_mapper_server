pub struct Response {
    proto: String,
    res_code: ResponseCode,
    content: String
}

impl Response {
    pub fn new(proto: String, res_code: ResponseCode, content: String) -> Self {
        Self { proto, res_code, content }
    }
    pub fn to_response(&self) -> String {
        let (code, msg) = self.res_code.to_tuple();
        let status_line = format!("{} {} {}", self.proto, code, msg);

        format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, self.content.len(), self.content)
    }
}

pub enum ResponseCode {
    OK,
    NotFound
}

impl ResponseCode {
    pub fn to_tuple(&self) -> (usize, String) {
        match self {
            ResponseCode::OK => (200, "OK".into()),
            ResponseCode::NotFound => (404, "Not Found".into())
        }
    }
}
