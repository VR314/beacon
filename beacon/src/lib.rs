use std::io;
pub struct Client {
    pub connection: ConnectionType,
    // TODO: make an async function run() to actually run the client
}

impl Client {
    pub async fn run(&mut self) {
        println!("Client running!")
    }
}

pub struct Server {
    pub connection: ConnectionType,
    // TODO: make an async function run() to actually run the client
}

impl Server {
    pub async fn run(&mut self) {
        println!("Server running!")
    }
}

// This is used to abstract the client and server to use different types of connections in a
// plug-and-play fashion.
pub enum ConnectionType {
    WebSocket(WebSocketConnection),
    // IPC(IPCConnection),
}

pub trait Connection {
    fn connect(&mut self) -> Result<(), io::Error>;
}

pub struct WebSocketConnection {
    pub address: String,
    pub port: u32,
}

impl WebSocketConnection {
    pub fn new(address: String, port: u32) -> Self {
        Self { address, port }
    }
}

impl Connection for WebSocketConnection {
    fn connect(&mut self) -> Result<(), io::Error> {
        println!("Connecting to {}:{}", self.address, self.port);
        // TODO: connect to web socket
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    //    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
