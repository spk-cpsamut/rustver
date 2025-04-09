use std::{
    collections::HashMap,
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

mod enums;
use enums::http_methods::http_method;
struct Request {
    method: http_method,
    endpoint: String,
    version: String,
    headers: HashMap<String, String>,
}

#[derive(Debug)]
struct RequestBuilder<'a> {
    method: Option<http_method>,
    endpoint: Option<&'a str>,
    version: Option<&'a str>,
    headers: HashMap<String, String>,
}

impl<'a> RequestBuilder<'a> {
    fn init() -> RequestBuilder<'a> {
        RequestBuilder {
            method: None,
            endpoint: None,
            version: None,
            headers: HashMap::new(),
        }
    }

    fn add_method(
        &'a mut self,
        method: Option<http_method>,
        endpoint: Option<&'a str>,
        version: Option<&'a str>,
    ) {
        self.method = method;
        self.endpoint = endpoint;
        self.version = version;
    }

    fn add_header_property(&mut self, key: &str, val: &str) {
        self.headers.insert(key.to_string(), val.to_string());
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let st: std::net::TcpStream = stream.unwrap();

        handle_connection(st)
    }
    println!("Hello, world!");

    let mut kaw = "hello".to_string();

    tranfrom(&mut kaw);
    tranfrom(&mut kaw);
    tranfrom(&mut kaw);

    print!("{}", kaw)
}

fn tranfrom(kaw: &mut String) {
    kaw.insert(0, 'G')
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .for_each(|line| handle_buffer(line));
    let content = fs::read_to_string("hello.html").unwrap();

    let status_line = "HTTP/1.1 200 OK";
    let length = content.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n {content}");
    stream.write_all(response.as_bytes()).unwrap()
}

fn handle_buffer(line: String) {
    let mut splited_line = line.split(":").collect::<Vec<&str>>();
    let mut candidate_request = RequestBuilder::init();
    match splited_line.get(1) {
        Some(val) => {
            handle_header_properties(splited_line.get(0).unwrap(), val, &mut candidate_request)
        }
        None => handle_http_method_header(splited_line.get(0).unwrap(), &mut candidate_request),
    }
}

fn handle_http_method_header<'a>(
    http_method: &'a str,
    candidate_request: &'a mut RequestBuilder<'a>,
) {
    let list = http_method.split(" ").collect::<Vec<&str>>();

    let method = map_http_method(list[0]);
    let endpoint = list.get(1).copied();
    let version = list.get(2).copied();

    candidate_request.add_method(method, endpoint, version);
}

fn handle_header_properties<'a>(
    key: &str,
    val: &str,
    candidate_request: &'a mut RequestBuilder<'a>,
) {
    candidate_request.add_header_property(key, val);
}

fn map_http_method(method: &str) -> Option<http_method> {
    match method {
        "GET" => return Some(http_method::GET),
        "POST" => return Some(http_method::POST),
        "PUT" => return Some(http_method::PUT),
        "DELETE" => return Some(http_method::DELETE),
        "PATCH" => return Some(http_method::PATCH),
        _ => return None,
    }
}
