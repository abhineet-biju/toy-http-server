use crate::request::Request;
use std::{
    io::{Read, Write},
    net::TcpListener,
};

pub struct Server {
    pub listener: TcpListener,
}

impl Server {
    // Create new TcpListener binding and initiate a port connection
    pub fn new(address: &str) -> Result<Self, std::io::Error> {
        let listener = TcpListener::bind(address)?;

        Ok(Self { listener })
    }

    // Main server loop
    pub fn run(&self) -> Result<(), std::io::Error> {
        println!("Server running on http://{}", self.listener.local_addr()?);

        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = [0u8; 1024];
                    let bytes_read = stream.read(&mut buffer)?;

                    if bytes_read == 0 {
                        continue;
                    }

                    match Request::parse(&buffer[0..bytes_read]) {
                        Ok(request) => {
                            println!("Parsed request: {:?}", request);

                            let response = "HTTP/1.1 200 OK\r\n\r\nHello from toy HTTP server!";
                            stream.write_all(response.as_bytes())?;
                            stream.flush()?;
                        }
                        Err(error) => {
                            eprintln!("Failed to parse request: {:?}", error);

                            let response = "HTTP/1.1 400 Bad Request\r\n\r\nBad Request";
                            stream.write_all(response.as_bytes())?;
                            stream.flush()?;
                        }
                    }
                }
                Err(error) => {
                    eprintln!("Failed to accept connection: {}", error);
                }
            }
        }

        Ok(())
    }
}
