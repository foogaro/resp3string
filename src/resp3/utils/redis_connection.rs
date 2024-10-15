use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::str;

// RedisConnection manages the actual TCP connection to Redis
pub struct RedisConnection {
    stream: TcpStream,
}

impl RedisConnection {
    pub fn new(address: &str) -> Self {
        let stream = TcpStream::connect(address).expect("Could not connect to Redis server");
        RedisConnection { stream }
    }

    pub fn send_command(&mut self, command: &str) -> String {
        self.stream.write_all(command.as_bytes()).expect("Failed to write to Redis server");
        self.stream.flush().expect("Failed to flush the stream");

        let mut buffer = [0; 512];
        let bytes_read = self.stream.read(&mut buffer).expect("Failed to read from Redis server");
        let response = str::from_utf8(&buffer[..bytes_read]).expect("Failed to parse Redis response");
        response.to_string()
    }
    pub fn close(&mut self) {
        self.stream.shutdown(Shutdown::Both).expect("shutdown call failed");
    }
}

impl Drop for RedisConnection {
    fn drop(&mut self) {
        println!("Dropping RedisConnection...");
        self.close();
    }
}