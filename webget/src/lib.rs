use std::error::Error;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};

pub fn get_url(host: &str, path: &str) -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect((host, 80))?;
    let message = format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n");
    stream.write_all(message.as_bytes())?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("{}", response);
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}
