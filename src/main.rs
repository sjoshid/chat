#[macro_use]
extern crate serde_derive;
extern crate websocket;

mod sync;
mod common;

use std::thread;
use std::sync::{Mutex, Arc};
use sync::server::ChatServer;
use common::WrapperSender;
use websocket::sync::Server;
use std::collections::HashMap;
use websocket::header::Cookie;
use websocket::sender::Writer;
use std::io::Write;

fn main() {
    let server = Server::bind("127.0.0.1:2794").unwrap();

    let mut sync_server = Arc::new(Mutex::new(ChatServer { clients: HashMap::new() })); ;
    for incoming_request in server.filter_map(Result::ok) {
        let cloned_sync_server = Arc::clone(&sync_server);
        thread::spawn(move || {
            if !incoming_request.protocols().contains(&"rust-websocket".to_string()) {
                incoming_request.reject().unwrap();
                return;
            }

            let h = incoming_request.request.headers.clone();
            let c = incoming_request.use_protocol("rust-websocket").accept().unwrap();
            let (mut receiver, mut sender) = c.split().unwrap();

            let user_id: String;
            {
                let cookie: &Cookie = match h.get() {
                    Some(c) => c,
                    None => {
                        panic!("Unable to determine user_id")
                    }
                };
                let v = &*cookie;
                let id = v[0].clone();
                user_id = id.split('=').collect::<Vec<&str>>().get(1).unwrap().to_string();
            }

            let wc2 = WrapperSender::new(user_id, sender);
            let mut mg = cloned_sync_server.lock().unwrap();
            let mut chat_server = &mut *mg;
            chat_server.add_client(wc2);

            for message in receiver.incoming_messages() {
                chat_server.send_message(message.unwrap());
            }
        });
    }
}
