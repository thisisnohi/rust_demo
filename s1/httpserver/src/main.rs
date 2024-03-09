mod server;
mod router;
mod handler;

use server::Server;

fn main() {
    println!("Hello, Server!");
    let server = Server::new("localhost:3000");
    server.run();
}
