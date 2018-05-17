extern crate websocket;
use websocket::client::sync::Client;
use websocket::stream::sync::TcpStream;
use websocket::stream::Stream;

pub struct ChatServer<S> where S: Stream {
        //existing_users: &[String]
        pub my_users: Vec<Client<S>>
}

impl ChatServer<TcpStream> {
    pub fn add_client(&mut self, client: Client<TcpStream>) {
        self.my_users.push(client);
        println!("{}", self.my_users.len());
    }
}