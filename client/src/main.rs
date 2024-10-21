use beacon::{
    client::Client,
    connection::{ConnectionType, WebSocketConnection},
};

#[tokio::main]
async fn main() {
    let client_conn = ConnectionType::WebSocket(WebSocketConnection::new(
        "wss://echo.websocket.events".to_owned(),
    ));
    let mut client = Client {
        connection: client_conn,
    };
    client.run().await;
}
