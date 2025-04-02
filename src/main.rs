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
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    print!(" 5555");
    let content = fs::read_to_string("hello.html").unwrap();
    print!("{:#?}", content);
    let status_line = "HTTP/1.1 200 OK";
    let length = content.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n {content}");

    print!("Response: {:#?}", response);
    stream.write_all(response.as_bytes()).unwrap()
}
