use std::{fs, thread};
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::time::Duration;
use server::ThreadPool;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let pool = ThreadPool::new(4);
        pool.execute(|| {
            handle_connection(stream);
        })
    }
}


fn handle_connection(mut stream: TcpStream) {
    //Создаем байтовый буффер
    let mut buffer = [0; 1024];
    //Заполняем буффре
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_code, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_code,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap() // очишаем байтоывй буффер
}
