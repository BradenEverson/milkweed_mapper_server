use std::{fs::{self, OpenOptions}, io::{BufRead, BufReader, BufWriter, Read, Write}, net::{TcpListener, TcpStream}};

use milkweed_mapper_server::{result::Result, loc::circle::Circle, tcp::request::{Method, RequestObject}};

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;

    for stream in listener.incoming() {
        let stream = stream?;

        handle_stream(stream)?;
    }
    Ok(())
}

fn handle_stream(mut stream: TcpStream) -> Result<()> {
    let mut buf_reader = BufReader::new(&mut stream);
    let request: Vec<_> = buf_reader
        .by_ref()
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let req_obj = RequestObject::new(request);

    match &req_obj.method {
        Method::GET => {
            let res = match req_obj.path.as_str() {
                "/" => HttpResponse::new("HTTP/1.1 200 OK", fs::read_to_string("html/map.html")?),
                _ => HttpResponse::new("HTTP/1.1 404 NOT FOUND", fs::read_to_string("html/404.html")?),
            };
            stream.write_all(&res.to_bytes())?;
        },
        Method::POST => {
            match req_obj.path.as_str() {
                "/collect-locs" => {
                    if let Some(content_length) = req_obj.headers.get("Content-Length") {
                        let content_length: usize = content_length.parse()?;
                        let mut body = vec![0; content_length];
                        buf_reader.read_exact(&mut body)?;

                        let circles: std::result::Result<Vec<Circle>, serde_json::Error> = serde_json::from_slice(&body);

                        if let Ok(circ) = circles {
                            let file = OpenOptions::new()
                                .create(true)
                                .write(true)
                                .truncate(true)
                                .open("check_locations.json")?;

                            let mut writer = BufWriter::new(file);

                            serde_json::to_writer(&mut writer, &circ)?;
                        }
                    }
                },
                _ => {
                    let res = HttpResponse::new("HTTP/1.1 404 NOT FOUND", fs::read_to_string("html/404.html")?);
                    stream.write_all(&res.to_bytes())?;
                }
            }
        }
    }

    Ok(())
}

pub struct HttpResponse {
    status: String,
    contents: String
}

impl HttpResponse {
    pub fn new(status: &str, contents: String) -> Self {
        Self { status: status.into(), contents }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        format!("{}\r\nContent-Length: {}\r\n\r\n{}", self.status, self.contents.len(), self.contents).as_bytes().to_vec()
    }
}
