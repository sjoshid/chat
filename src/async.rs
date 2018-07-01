mod async {
    extern crate websocket;

    use common;
    use std::collections::HashMap;
    use websocket::client::sync::Client;
    use websocket::OwnedMessage;
    use websocket::stream::Stream;
    use websocket::stream::sync::AsTcpStream;

    pub struct AsyncChatServer {
        pub clients: HashMap<String, common::WrapperClient>
    }

    impl AsyncChatServer {
        ppub fn init(){

        }

        pub fn add_client(&mut self, client: common::WrapperClient) {
            self.clients.insert(client.get_username().to_string(), client);
            //println!("{}", self.clients.len());
        }

        pub fn send_message_to_last(&mut self) {

        }
    }
}