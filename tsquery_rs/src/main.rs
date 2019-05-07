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

#[derive(Serialize)]
struct Output {
    pub status: u32,
    pub nodes: Vec<TreeNode>,
}

fn main() {
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
    serde_json::to_string(&output_struct).unwrap()
}
