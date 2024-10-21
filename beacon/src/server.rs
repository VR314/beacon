use crate::connection::ConnectionType;

pub struct Server {
    pub connection: ConnectionType,
}

impl Server {
    pub async fn run(&mut self) {
        println!("Server running!")
    }
}
