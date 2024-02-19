use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::domain::Repository;

type Handler<R> = fn(Request, &mut R) -> String;

pub type Request = String;

fn handle_read(mut stream: &TcpStream) -> String {
    let mut buf = [0u8; 4096 * 16];
    stream.read(&mut buf).unwrap();
    let req_str = String::from_utf8_lossy(&buf);
    req_str.to_string()
}

fn handle_write(mut stream: TcpStream, response_json: String) {
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/json; charset=UTF-8\r\n\r\n{}\r\n",
        response_json
    );
    match stream.write(response.as_bytes()) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn parse_request(input: String) -> Request {
    // println!("request = {}", input);

    let mut content_length = 0;
    for line in input.lines() {
        if let Some((header, val)) = line.split_once(":") {
            // println!("[{}] -> [{}]", header, val);
            if header == "Content-Length" {
                content_length = val.trim().parse::<usize>().unwrap();
            }
        }
    }
    // println!("content length = {:?}", content_length);

    let body = &input.lines().last().unwrap()[..content_length];
    body.to_string()
}

fn handle_client<R: Repository>(stream: TcpStream, handler: Handler<R>, repo: &mut R) {
    let raw_request = handle_read(&stream);
    let request = parse_request(raw_request);
    let response_json = handler(request, repo);
    handle_write(stream, response_json);
}

pub fn serve<R: Repository>(port: usize, handler: Handler<R>, repo: &mut R) {
    let addres = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(addres).unwrap();
    println!("Listening for connections on port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, handler, repo);
            }
            Err(e) => {
                println!("Unable to connect: {}", e);
            }
        }
    }
}
