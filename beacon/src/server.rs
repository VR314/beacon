use crate::connection::{ConnectionType, WebSocketConnection};

use futures_util::{SinkExt, StreamExt};
use once_cell::sync::Lazy;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tokio_tungstenite::tungstenite::Message;

/// The HashMap that holds global state for the program. Telemetry is pulled from this, and commands can modify these.
/// The HashMap has String keys (akin to the name of the data channel), and `std::any::Any` as values, which get downcasted to the desired type in the `get_value` function.
static MAP: Lazy<Arc<RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>>> = Lazy::new(|| {
    println!("Initializing MAP");
    Arc::new(RwLock::new(HashMap::new()))
});

pub async fn get_value<V: 'static + Clone>(key: String) -> Option<V> {
    let map_r = MAP.read().await;
    let value = map_r.get(&key);

    match value {
        Some(t) => {
            let dc = t.downcast_ref::<V>();
            match dc {
                // Clone the value from the HashMap and return it
                Some(v) => Some((*v).clone()),
                None => None,
            }
        }
        None => None,
    }
}

pub async fn insert_value<V: 'static + Send + Sync>(key: String, value: V) {
    let mut map_w = MAP.write().await;
    map_w.insert(key, Box::new(value));
}

pub struct Server {
    pub connection: ConnectionType,
}

impl Server {
    pub async fn run(&mut self) {
        println!("Server running!");
        match &self.connection {
            ConnectionType::WebSocket(ws) => {
                Server::run_ws_client(ws).await;
            }
        }
    }

    async fn run_ws_client(ws_conn: &WebSocketConnection) {
        // Create the event loop and TCP listener we'll accept connections on.
        let try_socket = TcpListener::bind(&ws_conn.url).await;
        let listener = try_socket.expect("Failed to bind");
        println!("Listening on: {}", &ws_conn.url);

        insert_value("test_key".to_owned(), "test_value".to_owned()).await;
        println!("{:?}", get_value::<String>("test_key".to_owned()).await);

        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(accept_connection(stream));
        }
    }
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("Connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    println!("New WebSocket connection: {}", addr);

    let (mut write, mut read) = ws_stream.split();
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                let response = handle_command(text).await;
                if let Err(e) = write.send(Message::Text(response)).await {
                    eprintln!("Error sending message: {}", e);
                    break;
                }
            }
            Ok(Message::Binary(bin)) => {
                println!("Received binary data: {:?}", bin);
            }
            Ok(Message::Close(_)) => {
                println!("Connection closed by peer: {}", addr);
                break;
            }
            Err(e) => {
                eprintln!("Error reading message: {}", e);
                break;
            }
            _ => {}
        }
    }
}

async fn handle_command(msg: String) -> String {
    // Process the message and return a response
    println!("Received command: {}", msg);
    // For example, return a simple response for now
    format!("Echo: {}", msg)
}
