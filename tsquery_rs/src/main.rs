extern crate serde;
extern crate serde_json;
extern crate telnet;
#[macro_use]
extern crate serde_derive;


mod channel;
mod client;
mod err;
mod general;
mod tree_node;
mod util;


use actix_web::{web, App, HttpRequest, HttpServer};
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tree_node::TreeNode;

#[derive(Serialize)]
struct Output {
    pub status: u32,
    pub nodes: Vec<TreeNode>,
}

pub struct TeamspeakData {
    pub nodes: Vec<TreeNode>,
    pub title: String,
}

fn get_ts_envs() -> Result<(String, String, String), env::VarError> {
    let host = env::var("TS_HOST")?;
    let username = env::var("TS_USERNAME")?;
    let password = env::var("TS_PASSWORD")?;
    Ok((host, username, password))
}

fn main() {
    if let Ok((ts_host, ts_username, ts_password)) = get_ts_envs() {
        let teamspeak_data_arc = Arc::new(Mutex::new(TeamspeakData {
            nodes: Vec::new(),
            title: "".into(),
        }));

        let teamspeak_data_arc_clone = teamspeak_data_arc.clone();
        let mut got_title = false;

        thread::spawn(move || loop {
            if let Ok(mut conn) = util::init_conn(&ts_host, &ts_username, &ts_password) {
                if let Ok(nodes) = tree_node::TreeNode::get_all(&mut conn) {
                    let mut teamspeak_data = teamspeak_data_arc_clone.lock().unwrap();
                    teamspeak_data.nodes = nodes;

                    if !got_title {
                        if let Ok(info) = general::ServerInfo::get(&mut conn) {
                            teamspeak_data.title = info.virtualserver_name;
                            got_title = true;
                        }
                    }
                }
            }
            thread::sleep(Duration::from_millis(2000));
        });
        http_server(teamspeak_data_arc);
    } else {
        panic!("please set TS_HOST, TS_USERNAME and TS_PASSWORD env");
    }
}

fn http_server(teamspeak_data: Arc<Mutex<TeamspeakData>>) {
    HttpServer::new(move || {
        App::new()
            .data(teamspeak_data.clone())
            .service(web::resource("/api/tsquery").to(index))
            .service(web::resource("/api/config").to(config))
    })
    .bind("0.0.0.0:3000")
    .unwrap()
    .run()
    .unwrap();
}

fn index(state: web::Data<Arc<Mutex<TeamspeakData>>>, _req: HttpRequest) -> String {
    let data = state.lock().unwrap();
    let output = Output {
        status: 200,
        nodes: data.nodes.clone(),
    };
    serde_json::to_string(&output).unwrap()
}

#[derive(Serialize)]
struct Config {
    pub page_title: String,
    pub teamspeak_title: String,
    pub teamspeak_url: String,
    pub twitch_channel: String,
}

fn config(state: web::Data<Arc<Mutex<TeamspeakData>>>, _req: HttpRequest) -> String {
    let data = state.lock().unwrap();
    let config = Config {
        page_title: "Wedelstaub".into(),
        teamspeak_title: data.title.clone(),
        teamspeak_url: "shnaky.com".into(),
        twitch_channel: "wedelstaub".into(),
    };
    serde_json::to_string(&config).unwrap()
}
