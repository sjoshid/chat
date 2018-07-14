extern crate websocket;
extern crate serde;
extern crate serde_json;

use common;

use websocket::client::sync::Client;
use websocket::stream::Stream;
use websocket::stream::sync::AsTcpStream;
use websocket::OwnedMessage;
use std::collections::HashMap;

pub struct ChatServer {
    pub clients: HashMap<String, common::WrapperClient>
}

impl ChatServer {
    pub fn add_client(&mut self, client: common::WrapperClient) {
        self.clients.insert(client.get_username().to_string(), client);
    }

    pub fn send_message(&mut self, message_from_sender: OwnedMessage) {
        let mfs = message_from_sender.unwrap();
        let incoming_message_details: common::MessageDetails = serde_json::from_str(mfs).unwrap(); //<-- deserialize

        let receiver_username = incoming_message_details.receiver_username;

        match self.clients.get(&receiver_username) {
            Some(receiver) => {
                let outgoing_message_details = common::MessageDetails{sender_username: String::from(""), receiver_username: String::from(""), message: incoming_message_details.message};

                let serialized_outgoing_message = serde_json::to_string(&outgoing_message_details).unwrap();
                receiver.send_message(OwnedMessage::Text(serialized_outgoing_message)).unwrap();
            },
            None => println!("Error because username not found in hashmap of server.")
        }
    }
}