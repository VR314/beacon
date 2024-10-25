use std::io;

// This is used to abstract the client and server to use different types of connections in a
// plug-and-play fashion.

#[derive(Clone, Debug)]
pub enum ConnectionType {
    WebSocket(WebSocketConnection),
    // IPC(IPCConnection),
}

pub trait Connection {
    fn connect(&mut self) -> Result<(), io::Error>;
}

#[derive(Clone, Debug)]
pub struct WebSocketConnection {
    pub url: String,
}

impl WebSocketConnection {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

impl Connection for WebSocketConnection {
    fn connect(&mut self) -> Result<(), io::Error> {
        println!("Connecting to WS at {}", self.url);
        // TODO: connect to web socket
        Ok(())
    }
}
