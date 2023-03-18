use std::io::{self, BufRead, Write};
use std::io::{BufReader, LineWriter};
use std::net::TcpStream;

pub mod executor;
pub mod requests;
pub mod responses;

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("www.google.com:80")?;
    let mut writer = LineWriter::new(stream.try_clone()?);
    let mut reader = BufReader::new(stream);

    writer.write(&"GET / HTTP/1.0\r\nHost: www.google.com\r\n\r\n".as_bytes())?;

    let mut line = String::new();

    reader.read_line(&mut line)?;
    for ln in reader.lines() {
        let actual = ln?;
        println!("{}", actual);
    }
    line.pop();
    println!("{}", line);
    Ok(())
}
