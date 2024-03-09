use std::{net::TcpListener, result};
use std::io::{Read,Write};

fn main() {
    
    let listenter = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");

    for stream in listenter.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established");
        
        let mut buffer = [0; 1024];
        
        stream.read(&mut buffer).unwrap();

        stream.write(&buffer).unwrap();

    }

}
