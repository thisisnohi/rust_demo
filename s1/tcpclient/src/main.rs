use std::net::TcpStream;
use std::io::{Read,Write};
use std::str;

fn main() {
    
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap();

    stream.write("hello".as_bytes()).unwrap();

    // [0;5]时正好读取到hello
    // let mut buffer = [0; 5];
    let mut buffer = [0; 1024];

    let size = stream.read(&mut buffer).unwrap();
    println!("read size:{}", size);
    
    let buffer = &buffer[0..size];

    println!("Reciver msg: {:?}", String::from_utf8(buffer.to_vec()));
    // println!("{:?}", str::from_utf8(&buffer, 0, size).unwrap());


}
