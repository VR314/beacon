pub struct Client {
    connection: ConnectionType,
    // TODO: make an async function run() to actually run the client
}

pub struct Server {
    connection: ConnectionType,
    // TODO: make an async function run() to actually run the client
}

// This is used to abstract the client and server to use different types of connections in a
// plug-and-play fashion.
pub enum ConnectionType {
    WebSocket(WebSocketConnection),
    // IPC(IPCConnection),
}

pub trait Connection {
    fn connect(&mut self);
}

pub struct WebSocketConnection {
    address: String,
    port: u32,
    connected: bool,
}

impl Connection for WebSocketConnection {
    fn connect(&mut self) {
        println!("Connecting to {}:{}", self.address, self.port);
        // TODO: connect to web socket
        self.connected = true;
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
