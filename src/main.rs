use std::net::TcpListener;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;
use std::fs;

fn main() {
    let listener= TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream:TcpStream){
    let buf_reader = BufReader::new(&stream);
    let http_request: String = buf_reader
        .lines().next().unwrap().unwrap();
    let (status, filename) =
    if http_request == "GET / HTTP/1.1"{
        ("HTTP/1.1 200 OK", "hello.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let res_content = fs::read_to_string(filename).unwrap();
    let length = res_content.len();
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{res_content}");
    stream.write_all(response.as_bytes()).unwrap();
}