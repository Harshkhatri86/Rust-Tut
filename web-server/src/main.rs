use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};
mod multithreaded_server;

fn main() {
    // initialize a tcp server , and unwrap won't show error
    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();

    //     handle_connection(stream)
    // }
    multithreaded_server::main();
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request :{http_request:#?}");

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}
