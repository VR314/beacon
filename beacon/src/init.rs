// The macro to define channels and packets
#[macro_export]
// The macro to define channels and packets
macro_rules! init {
    (
        channels: [
            $(
                { name: $ch_name:literal, type: $ch_type:ident,
                    default_value: $ch_default:expr, perms: [$($ch_perm:ident),*] }
            ),* $(,)?
        ],
        packets: [
            $(
                { name: $pkt_name:literal, channels: [$($pkt_ch:literal),*],
                    rate: $pkt_rate:expr }
            ),* $(,)?
        ]
    ) => {
        {
            // TODO: introduce checking for:
            //      - all channel names are unique
            //      - packets are smaller than the max packet size
            //      - etc.

            let channels: Vec<Box<beacon::Channel>> = vec![
                $(
                Box::new(beacon::Channel {
                        name: $ch_name,
                        ch_type: beacon::BeaconTypes::$ch_type,
                        default_value: $ch_default,
                        perms: vec![$(beacon::ChannelPermissions::$ch_perm),*],
                    })
                ),*
            ];


            let mut channel_map = std::collections::HashMap::new();
            $(
                channel_map.insert($ch_name, Box::new(beacon::Channel {
                    name: $ch_name,
                    ch_type: $ch_type,
                    default_value: $ch_default,
                    perms: vec![$($ch_perm),*],
                }));
            )*

            let packets: Vec<beacon::Packet> = vec![
                $(
                    beacon::Packet {
                        name: $pkt_name,
                        channels: vec![$($pkt_ch),*],
                        rate: $pkt_rate,
                    }
                ),*
            ];

            let mut packet_map = std::collections::HashMap::new();
            $(
                packet_map.insert($pkt_name, Box::new(beacon::Packet {
                        name: $pkt_name,
                        channels: vec![$($pkt_ch),*],
                        rate: $pkt_rate,
                }));
            )*

            (channels, channel_map, packets, packet_map)
        }
    };
}
