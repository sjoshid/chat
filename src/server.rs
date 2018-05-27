extern crate websocket;
use client;

use websocket::client::sync::Client;
use websocket::stream::Stream;
use websocket::stream::sync::AsTcpStream;
use websocket::OwnedMessage;
use std::collections::HashMap;

pub struct ChatServer {
    pub clients: HashMap<String, client::WrapperClient>
}

impl ChatServer {
    pub fn add_client(&mut self, client: client::WrapperClient) {
        self.clients.insert(client.get_username().to_string(), client);
        println!("{}", self.clients.len());
    }

    pub fn send_message_to_last(&mut self) {

    }
}