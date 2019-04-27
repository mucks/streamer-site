use std::collections::HashMap;
use telnet::Telnet;
use telnet::TelnetEvent::{self, *};

pub fn init_conn() -> Telnet {
    let mut conn = Telnet::connect("shneky.com:10011", 4096).unwrap();
    let _ = conn.read().unwrap();

    conn.write(b"login shneky 9AsefCzT\r\n").unwrap();
    let _ = conn.read().unwrap();
    conn.write(b"use sid=1\r\n").unwrap();
    let _ = conn.read().unwrap();
    conn
}

pub fn telnet_event_to_hashmap(event: &TelnetEvent) -> Option<Vec<HashMap<String, String>>> {
    match event {
        Data(data) => {
            let mut maps = Vec::new();

            let channels_str = String::from_utf8((*data).to_vec()).unwrap();
            let channels: Vec<&str> = channels_str.split("|").collect();
            for channel in channels {
                let mut map = HashMap::new();

                let kv_pairs_str: Vec<&str> = channel.split_whitespace().collect();
                for kv_pair_str in kv_pairs_str {
                    let kv_pair: Vec<&str> = kv_pair_str.split("=").collect();
                    if kv_pair.len() == 2 {
                        let key = kv_pair[0].to_owned();
                        let val = kv_pair[1].to_owned();
                        let entry = map.entry(key).or_insert(val.clone());
                        *entry = val;
                    }
                }
                maps.push(map);
            }
            Some(maps)
        }
        _ => None,
    }
}
