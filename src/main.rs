mod MyServer;

extern crate websocket;

use std::thread;
use websocket::OwnedMessage;
use websocket::sync::Server;

fn main() {
    let server = Server::bind("127.0.0.1:2794").unwrap();
    let mut chat_server = MyServer::ChatServer{my_users: Vec::new()};

    for request in server.filter_map(Result::ok) {

        if !request.protocols().contains(&"rust-websocket".to_string()) {
            request.reject().unwrap();
            return;
        }

        let mut client = request.use_protocol("rust-websocket").accept().unwrap();
        chat_server.add_client(client);
        chat_server.send_message_to_last();
    }
}


// fn main() {
//     // let mut users = MyServer::ChatServer{my_users: vec![String::from("")]};
//     let mut users = MyServer::ChatServer{my_users: Vec::new()};
//     users.add_user(String::from("Morgan"));
//     println!("First user is {}", users.my_users[0]);
// }