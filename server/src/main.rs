use beacon::{
    connection::{ConnectionType, WebSocketConnection},
    server::Server,
};

#[tokio::main]
async fn main() {
    let server_conn = ConnectionType::WebSocket(WebSocketConnection::new("localhost".to_owned()));
    let mut server = Server {
        connection: server_conn,
    };
    server.run().await;
}
