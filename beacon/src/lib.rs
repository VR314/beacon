pub mod client;
pub mod connection;
pub mod init;
pub mod server;

// Define the necessary enums
#[derive(Debug, Clone)]
pub enum BeaconTypes {
    U8,
    // Add other types as needed
}

#[derive(Debug, Clone)]
pub enum ChannelPermissions {
    Commandable,
    // Add other permissions as needed
}

// Define the Channel struct
#[derive(Debug, Clone)]
pub struct Channel {
    pub name: &'static str,
    pub ch_type: BeaconTypes,
    pub default_value: Option<u8>, // Assuming u8 for now, change as needed
    pub perms: Vec<ChannelPermissions>,
}

// Define the Packet struct
#[derive(Debug)]
pub struct Packet {
    pub name: &'static str,
    pub channels: Vec<&'static str>, // Channels are stored in Box to match your example
    pub rate: u32,
}

#[cfg(test)]
mod tests {
    //    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
