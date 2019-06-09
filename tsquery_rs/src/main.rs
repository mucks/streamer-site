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

use std::thread;
use std::time::Duration;
use tree_node::TreeNode;

#[derive(Serialize)]
struct Output {
    pub status: u32,
    pub nodes: Vec<TreeNode>,
}

use actix_web::{web, App, HttpRequest, HttpServer};

use std::env;
use std::sync::{Arc, Mutex};

pub struct TeamspeakData {
    pub nodes: Vec<TreeNode>,
}

fn get_ts_envs() -> Result<(String, String, String), env::VarError> {
    let host = env::var("TS_HOST")?;
    let username = env::var("TS_USERNAME")?;
    let password = env::var("TS_PASSWORD")?;
    Ok((host, username, password))
}

fn main() {
    if let Ok((ts_host, ts_username, ts_password)) = get_ts_envs() {
        let teamspeak_data_arc = Arc::new(Mutex::new(TeamspeakData { nodes: Vec::new() }));

        let teamspeak_data_arc_clone = teamspeak_data_arc.clone();

        thread::spawn(move || loop {
            if let Ok(mut conn) = util::init_conn(&ts_host, &ts_username, &ts_password) {
                if let Ok(nodes) = tree_node::TreeNode::get_all(&mut conn) {
                    let mut teamspeak_data = teamspeak_data_arc_clone.lock().unwrap();
                    teamspeak_data.nodes = nodes;
                }
            }
            thread::sleep(Duration::from_millis(2000));
        });

        HttpServer::new(move || {
            App::new()
                .data(teamspeak_data_arc.clone())
                .service(web::resource("/api/tsquery").to(index))
        })
        .bind("0.0.0.0:3000")
        .unwrap()
        .run()
        .unwrap();
    } else {
        panic!("please set TS_USERNAME and TS_PASSWORD env");
    }
}

fn index(state: web::Data<Arc<Mutex<TeamspeakData>>>, _req: HttpRequest) -> String {
    let data = state.lock().unwrap();
    let output = Output {
        status: 200,
        nodes: data.nodes.clone(),
    };
    serde_json::to_string(&output).unwrap()
}
