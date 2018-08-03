#[macro_use]
extern crate serde_derive;
extern crate websocket;

mod incomplete_server;
mod common;

use websocket::sync::Server;
use std::collections::HashMap;
use websocket::header::Cookie;
use websocket::sender::Writer;
use std::io::Write;

fn main() {
    let server = Server::bind("127.0.0.1:2794").unwrap();

    //let mut first_request = true;
    let mut sync_server = incomplete_server::ChatServer { clients: HashMap::new() };
    for incoming_request in server.filter_map(Result::ok) {
        if !incoming_request.protocols().contains(&"rust-websocket".to_string()) {
            incoming_request.reject().unwrap();
            return;
        }

        let h = incoming_request.request.headers.clone();
        let c = incoming_request.use_protocol("rust-websocket").accept().unwrap();
        let (mut receiver, mut sender) = c.split().unwrap();

        let user_id: String;
        {
            //let headrs = c.headers();
            let cookie: &Cookie = match h.get() {
                Some(c) => c,
                None => {
                    println!("Unable to determine user_id");
                    continue;
                }
            };
            let v = &*cookie;
            let id = v[0].clone();
            user_id = id.split('=').collect::<Vec<&str>>().get(1).unwrap().to_string();
        }

        let wc2 = common::WrapperSender::new(user_id, sender);
        sync_server.add_client(wc2);

        for message in receiver.incoming_messages() {
            //println!("received");
            eprintln!("Received {:?} ", message.unwrap());
            //let raw_message_from_client = message.unwrap();
        }
    }
}
