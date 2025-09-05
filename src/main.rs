use std::io::{Read, Write};
use std::error::Error;
use std::net::TcpStream;

fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = TcpStream::connect("example.org:80")?;
    let msg = b"GET / HTTP/1.1\r\nHost: example.org\r\n\r\n";
    let mut response_buf = vec![0; 1024 * 1024];
    stream.write_all(msg)?;
    let bytes_read = stream.read(&mut response_buf)?;
    println!(
        "Response: {}",
        String::from_utf8_lossy(&response_buf[0..bytes_read])
    );
    println!("Connected to the server!");
    Ok(())
}
