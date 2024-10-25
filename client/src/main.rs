use beacon::{
    client::Client,
    connection::{ConnectionType, WebSocketConnection},
    init,
    BeaconTypes::*,
    ChannelPermissions::*,
};
#[tokio::main]
async fn main() {
    let client_conn =
        ConnectionType::WebSocket(WebSocketConnection::new("ws://127.0.0.1:8080".to_owned()));
    let mut client = Client {
        connection: client_conn,
    };
    let (channels, channel_map, packets, packet_map) = init!(
        channels: [
            {name: "c", type: U8, default_value: None, perms: [Commandable]},
            {name: "c2", type: U8, default_value: None, perms: [Commandable]}
        ],
        packets: [
            {name: "p1", channels: ["c", "c2"], rate: 10}
        ]
    );

    // Debug print the generated channels and packets
    println!("{:?}", channels);
    println!("{:?}", channel_map);
    println!("{:?}", packets);
    println!("{:?}", packet_map);

    client.run().await;
}
