use std::{error::Error, fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

use milkweed_mapper_server::tcp::request::{Method, RequestObject};

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:7878")?;

    for stream in listener.incoming() {
        let stream = stream?;

        handle_stream(stream)?;
    }
    Ok(())
}

fn handle_stream(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let buf_reader = BufReader::new(&mut stream);
    let request: Vec<_> = buf_reader
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
            println!("{:?}", &req_obj.headers);
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
