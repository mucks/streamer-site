extern crate serde;
extern crate serde_json;
extern crate telnet;
#[macro_use]
extern crate serde_derive;

mod channel;
mod client;
mod err;
mod tree_node;
mod util;

use actix_web::{http::Method, server, App, HttpRequest};
use tree_node::TreeNode;
use std::fs::File;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

#[derive(Serialize)]
struct Output {
    pub status: u32,
    pub nodes: Vec<TreeNode>,
}

fn save_ts() {
    let mut conn = util::init_conn();
    let output_struct = match tree_node::TreeNode::get_all(&mut conn) {
        Ok(nodes) => Output {
            status: 200,
            nodes: nodes,
        },
        Err(error) => {
            println!("{:?}", error);
            Output {
                status: 500,
                nodes: Vec::new(),
            }
        }
    };
    let data = serde_json::to_string(&output_struct).unwrap();
    let mut f = File::create("teamspeak.json").unwrap();
    f.write_all(data.as_bytes()).unwrap();
}

fn main() {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(2000));
            save_ts();
        }
    });
    server::new(|| {
        App::new()
            .prefix("/api")
            .resource("/tsquery", |r| r.method(Method::GET).f(index))
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run()
}

fn index(_req: &HttpRequest) -> String {
    let mut f = File::open("teamspeak.json").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
}
