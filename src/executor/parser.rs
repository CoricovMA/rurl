use std::{
    io::{self, Write},
    net::TcpStream,
};

use crate::requests::requests::Request;
use crate::responses::responses::Response;

pub struct Streamer {
    reader: io::BufReader<TcpStream>,
    writer: io::LineWriter<TcpStream>,
}

impl Streamer {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        let writer = io::LineWriter::new(stream.try_clone()?);
        let reader = io::BufReader::new(stream);
        Ok(Self { reader, writer })
    }

    pub fn send_request(&mut self, request: &Request) -> io::Result<()> {
        self.writer.write(&request.bytes())?;
        self.writer.write(&['\n' as u8])?;
        Ok(())
    }

    pub fn read_response(&mut self) -> io::Result<()> {
        Ok(())
    }
}
