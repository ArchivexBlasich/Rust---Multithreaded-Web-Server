use std::{fs, io::{BufRead, BufReader, Write}, net::{TcpListener, TcpStream}};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    // This buffer the HTTP request of rhe browser, each line is a element of the vector http_request
    /* 
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    */
    // println!("Request: {http_request:#?}"); // Shows the HTTP request from the browser

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contens = fs::read_to_string(filename).unwrap();
    let length = contens.len();
    
    let response = 
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contens}");

    stream.write_all(response.as_bytes()).unwrap();
}
