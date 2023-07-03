use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use server::ThreadPool;

fn main() {
    // listen on localhost and port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // create a thread pool
    let pool = ThreadPool::new(5);

    // call the incoming method on listener which will give iterator over connections recieved on listener in form of tcp stream
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    // store data to be read in buffer variable
    let mut buffer = [0; 1024];
    // read the data from buffer
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    // check wheather it is get request or any other request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 200 OK", "404.html")
    };
    //read the contents of 404.html
    let contents = fs::read_to_string(filename).unwrap();
    // return the response

    let response = format!(
        "{}Content-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    // write the response
    stream.write(response.as_bytes()).unwrap();
    // flush the stream
    stream.flush().unwrap()
}
