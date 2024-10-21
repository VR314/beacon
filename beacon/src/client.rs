use crate::connection::ConnectionType;

pub struct Client {
    pub connection: ConnectionType,
}

impl Client {
    pub async fn run(&mut self) {
        println!("Client running!")
    }
}
