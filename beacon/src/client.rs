use crate::connection::{ConnectionType, WebSocketConnection};

use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

pub struct Client {
    pub connection: ConnectionType,
}

impl Client {
    pub async fn run(&mut self) {
        println!("Client running!");
        match &self.connection {
            ConnectionType::WebSocket(ws) => {
                Client::run_ws_client(ws).await;
            }
        }
    }

    async fn run_ws_client(ws_conn: &WebSocketConnection) {
        let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
        tokio::spawn(read_stdin(stdin_tx));

        let (ws_stream, _) = connect_async(&ws_conn.url)
            .await
            .expect("Failed to connect");
        println!("WebSocket handshake has been successfully completed");

        let (write, read) = ws_stream.split();

        let stdin_to_ws = stdin_rx.map(Ok).forward(write);
        let ws_to_stdout = {
            read.for_each(|message| async {
                let data = message.unwrap().into_data();
                tokio::io::stdout().write_all(&data).await.unwrap();
            })
        };

        pin_mut!(stdin_to_ws, ws_to_stdout);
        future::select(stdin_to_ws, ws_to_stdout).await;
    }
}

// Our helper method which will read data from stdin and send it along the
// sender provided.
async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        // Convert the buffer to a String
        if let Ok(text) = String::from_utf8(buf) {
            // Send the text as a WebSocket text message
            tx.unbounded_send(Message::text(text)).unwrap();
        } else {
            eprintln!("Received invalid UTF-8 input");
        }
    }
}
