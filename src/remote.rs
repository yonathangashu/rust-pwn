use std::net::{TcpStream, ToSocketAddrs};
use std::io::{self, Read, Write};

pub struct Remote {
    stream: TcpStream,
}

impl Remote {
    pub fn new(addr: impl ToSocketAddrs) -> Result<Self, std::io::Error> {
        let stream = TcpStream::connect(addr)?;
        Ok(Self { stream })
    }

    pub fn send(&mut self, data: &[u8]) -> Result<usize, std::io::Error> {
        self.stream.write(data)
    }

    pub fn send_all(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        self.stream.write_all(data)
    }

    pub fn recv(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.stream.read(buf)
    }
}

impl Read for Remote {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf)
    }
}

impl Write for Remote {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
