use std::error::Error;
use std::io::{Read, Write};
mod remote;
use remote::*;

fn main() -> Result<(), Box<dyn Error>> {
    let mut remote = Remote::new("example.org:80")?;
    let msg = b"GET / HTTP/1.1\r\nHost: example.org\r\n\r\n";
    let mut response_buf = vec![0; 1024 * 1024];
    //let bytes_written = 
    remote.write_all(msg)?;
    //println!("Payload Size: {}", bytes_written);
    let bytes_read = remote.read(&mut response_buf)?;
    println!(
        "Response: {}",
        String::from_utf8_lossy(&response_buf[0..bytes_read])
    );
    println!("Connected to the server!");
    Ok(())
}
