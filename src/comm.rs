use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const IP_PORT: &str = "192.168.178.23:56789";

pub fn send_local(message: &String) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(IP_PORT)?;
    stream.write_all(message.as_bytes())?;
    Ok(())
}

pub fn listen_local() -> std::io::Result<String> {
    let listener = TcpListener::bind(IP_PORT)?;

    for stream in listener.incoming() {
        let mut buffer = [0; 2048];
        if let Ok(bytes_read) = stream.unwrap().read(&mut buffer) {
            return Ok(String::from_utf8_lossy(&buffer[..bytes_read]).into_owned());
        }
    }
    Ok(String::new())
}
