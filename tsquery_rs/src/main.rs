extern crate serde;
extern crate serde_json;
extern crate telnet;
extern crate ws;
#[macro_use]
extern crate serde_derive;

mod channel;
mod client;
mod tree_node;
mod util;

use std::sync::{Arc, Mutex};
use ws::listen;

fn main() {
    run();
}

fn run() {
    let main_conn_arc = Arc::new(Mutex::new(util::init_conn()));

    listen("127.0.0.1:3012", |out| {
        let conn_arc = main_conn_arc.clone();
        move |_msg| {
            let mut conn = conn_arc.lock().unwrap();
            let nodes = tree_node::TreeNode::get_all(&mut conn);
            let output = serde_json::to_string(&nodes).unwrap();
            let m: ws::Message = ws::Message::text(output);
            out.send(m)
        }
    })
    .unwrap();
}
