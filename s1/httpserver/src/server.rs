use crate::router::Router;
use http::httprequest::HttpRequest;
use std::{io::Read, net::TcpListener};

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }

    pub fn run(&self) {
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running on {}", self.socket_addr);

        for stream in connection_listener.incoming() {
            let mut stream: std::net::TcpStream = stream.unwrap();
            println!("Connection established");

            let mut read_buffer = [0; 200];

            stream.read(&mut read_buffer).unwrap();

            let content = String::from_utf8(read_buffer.to_vec());

            println!("content:{:?}", content);

            let req: HttpRequest = content.unwrap().into();

            println!("{:?}", req);
            // 分发路由
            Router::route(req, &mut stream);
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        println!("this is test!");
    }
}
