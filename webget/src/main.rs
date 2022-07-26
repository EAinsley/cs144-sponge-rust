use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

fn main() {
    let url = r"cs144.keithw.org";
    let path = r"/nph-hasher/xyzzy";
    let mut stream = TcpStream::connect(url.to_owned() + ":80").expect("cant connet to the server");
    let message = format!("GET {path} HTTP/1.1\r\nHost: {url}\r\nConnection: close\r\n\r\n");
    stream
        .write_all(message.as_bytes())
        .expect("error when sending message");
    println!("Message sent:\n{}", message);
    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .expect("error when reading response");
    println!("{}", response);
    stream
        .shutdown(Shutdown::Both)
        .expect("problem with shutdown");
}
