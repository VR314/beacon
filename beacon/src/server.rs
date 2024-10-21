use crate::connection::{ConnectionType, WebSocketConnection};

use std::{env, io::Error};

use futures_util::{future, StreamExt, TryStreamExt};
use tokio::net::{TcpListener, TcpStream};

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

        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(accept_connection(stream));
        }
    }
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    println!("New WebSocket connection: {}", addr);

    let (write, read) = ws_stream.split();
    // We should not forward messages other than text or binary.
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .forward(write)
        .await
        .expect("Failed to forward messages")
}
