extern crate websocket;
use websocket::client::sync::Client;
use websocket::stream::Stream;
use websocket::stream::sync::AsTcpStream;
use websocket::OwnedMessage;

pub struct ChatServer<S> where S: AsTcpStream + Stream {
        pub my_users: Vec<Client<S>>
}

impl<S> ChatServer<S> where S: AsTcpStream + Stream {
    pub fn add_client(&mut self, client: Client<S>) {
        self.my_users.push(client);
        println!("{}", self.my_users.len());
    }

    pub fn send_message_to_last(&mut self) {
        let mut last_client = self.my_users.last_mut().unwrap();
        let last_ip = last_client.peer_addr().unwrap();

        let message = OwnedMessage::Text(last_ip.to_string());

        last_client.send_message(&message).unwrap();
    }
}