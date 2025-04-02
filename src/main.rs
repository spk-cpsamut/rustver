use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let st: std::net::TcpStream = stream.unwrap();

        handle_connection(st)
    }
    println!("Hello, world!");
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

    match splited_line.get(1) {
        Some(val) => handle_header_properties(splited_line.get(0).unwrap(), val),
        None => handle_http_method_header(splited_line.get(0).unwrap()),
    }
}

fn handle_http_method_header(http_method: &str) {
    let list = http_method.split(" ").collect::<Vec<&str>>();

    let method = list[0];
    let endpoint = list[1];
    let version = list[2];

    print!("method: {:#?}", method);
    print!("endpoint: {:#?}", endpoint);
    print!("version: {:#?}", version);
}

fn handle_header_properties(key: &str, val: &str) {
    print!("key: {}", key);
    print!("val: {}", val);
}
