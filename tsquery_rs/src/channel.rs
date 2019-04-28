use crate::err::Error;
use crate::util;
use std::collections::HashMap;
use telnet::Telnet;

#[derive(Default, Debug)]
pub struct Channel {
    pub total_clients: u32,
    pub channel_needed_subscribe_power: u32,
    pub cid: u32,
    pub pid: u32,
    pub channel_order: u32,
    pub channel_name: String,
}

impl Channel {
    pub fn get_all(conn: &mut Telnet) -> Result<Vec<Channel>, Error> {
        conn.write(b"channellist\r\n")?;
        let event = conn.read()?;
        if let Some(maps) = util::telnet_event_to_hashmap(&event) {
            Ok(maps.iter().map(|map| Channel::from(map)).collect())
        } else {
            Err(Error::NoneError)
        }
    }
}

impl From<&HashMap<String, String>> for Channel {
    fn from(map: &HashMap<String, String>) -> Self {
        let mut channel = Channel::default();
        channel.channel_name = "not found".into();
        for (k, v) in map.iter() {
            println!("{}: {}", k, v);
            if k.as_str() == "channel_name" {
                channel.channel_name = v.to_owned();
            } else {
                let mut temp = 0;
                let field = match k.as_str() {
                    "total_clients" => &mut channel.total_clients,
                    "channel_needed_subscribe_power" => &mut channel.channel_needed_subscribe_power,
                    "cid" => &mut channel.cid,
                    "pid" => &mut channel.pid,
                    "channel_order" => &mut channel.channel_order,
                    _ => &mut temp,
                };
                *field = v.parse().unwrap_or(0);
            }
        }
        channel
    }
}
