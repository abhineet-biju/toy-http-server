mod server;

use server::Server;

fn main() -> Result<(), std::io::Error> {
    let server = Server::new("127.0.0.1:8080")?;
    server.run()
}
