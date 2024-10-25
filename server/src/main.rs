use beacon::{
    connection::{ConnectionType, WebSocketConnection},
    init,
    server::Server,
    BeaconTypes::*,
    ChannelPermissions::*,
};

#[tokio::main]
async fn main() {
    let server_conn =
        ConnectionType::WebSocket(WebSocketConnection::new("127.0.0.1:8080".to_owned()));

    let mut server = Server {
        connection: server_conn,
    };

    let server_clone = server.clone();

    let (channels, channel_map, packets, packet_map) = init!(
        channels: [
            {name: "c", type: U8, default_value: None, perms: [Commandable]},
            {name: "c2", type: U8, default_value: None, perms: [Commandable]}
        ],
        packets: [
            {name: "p1", channels: ["c", "c2"], rate: 10}
        ]
    );

    // TOOD: perform a handshake to verify that the init! data is the same on both sides
    // Could use serde to send over all the data, but could be costly. Instead, prefer to do some form of hashing...

    // Debug print the generated channels and packets
    println!("{:?}", channels);
    println!("{:?}", channel_map);
    println!("{:?}", packets);
    println!("{:?}", packet_map);

    tokio::spawn(async move {
        loop {
            server_clone.run_tlm().await;
        }
    });

    server.run().await;
}
