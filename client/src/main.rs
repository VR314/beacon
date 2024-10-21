use beacon::{
    client::Client,
    connection::{ConnectionType, WebSocketConnection},
};

#[tokio::main]
async fn main() {
    let client_conn =
        ConnectionType::WebSocket(WebSocketConnection::new("ws://127.0.0.1:8080".to_owned()));
    let mut client = Client {
        connection: client_conn,
    };
    client.run().await;
}
