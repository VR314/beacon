use beacon::{
    client::Client,
    connection::{ConnectionType, WebSocketConnection},
};

#[tokio::main]
async fn main() {
    let client_conn =
        ConnectionType::WebSocket(WebSocketConnection::new("localhost".to_owned(), 1001));
    let mut client = Client {
        connection: client_conn,
    };
    client.run().await;
}
