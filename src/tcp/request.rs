use std::collections::HashMap;

pub struct RequestObject {
    pub method: Method,
    pub path: String,
    pub proto: String,
    pub headers: HashMap<String, String>,
}

impl RequestObject {
    pub fn new(request: Vec<String>) -> Self {
        let first_lines = request[0].split(" ").collect::<Vec<&str>>();
        let method = match first_lines[0] {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::GET // bad but fix later
        };

        let path = first_lines[1].to_string();
        let proto = first_lines[2].to_string();

        let mut headers = HashMap::new();

        for i in 1..request.len() {
            let split_str = request[i].split(": ").collect::<Vec<&str>>();
            headers.insert(split_str[0].to_string(), split_str[1].to_string());   
        }

        Self { method, path, proto, headers }
    }
}


#[derive(Debug)]
pub enum Method {
    GET,
    POST,
    //etc
}
