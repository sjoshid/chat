//use std::collections::HashMap;

pub struct ChatServer {
        //existing_users: &[String]
        pub my_users: Vec<String>
}

impl ChatServer {
    pub fn add_user(&mut self, username: String) {
        println!("Adding user {}", username);    
        self.my_users.push(username);
    }
}