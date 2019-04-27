use crate::client::Client;
use std::collections::HashMap;
use telnet::Telnet;

#[derive(Serialize, Debug, Clone)]
pub struct ChannelOutput {
    pub name: String,
    pub children: Vec<ChannelOutput>,
    pub cid: u32,
    pub pid: u32,
    pub clients: Vec<Client>,
}

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
    pub fn get_all(conn: &mut Telnet) -> Vec<Channel> {
        conn.write(b"channellist\r\n").unwrap();
        let event = conn.read().unwrap();
        let maps = crate::util::telnet_event_to_hashmap(&event).unwrap();
        maps.iter().map(|map| Channel::from(map)).collect()
    }

    pub fn get_output(mut conn: &mut Telnet) -> Vec<ChannelOutput> {
        let clients = crate::client::Client::get_all(&mut conn);
        let channels = crate::channel::Channel::get_all(&mut conn);
        let mut channel_outputs = Vec::new();
        /*
        for chan in &channels {
            if chan.pid != 0 {
                if let Some(out) = channel_outputs.iter_mut().find(|o| o.cid == chan.pid) {
                    out.children.push(channel_to_out(&chan, &clients));
                }
            }
        }
        */

        for chan in &channels {
            channel_outputs.push(channel_to_out(&chan, &clients));
            for out in &mut channel_outputs {
                if chan.pid == out.cid {
                    out.children.push(channel_to_out(&chan, &clients));
                }
                for out_child in &mut out.children {
                    if chan.pid == out_child.cid {
                        out_child.children.push(channel_to_out(&chan, &clients));
                    }
                }
            }
        }

        channel_outputs
            .iter()
            .filter(|out| out.pid == 0)
            .map(|out| out.clone())
            .collect()
    }
}

fn channel_to_out(chan: &Channel, clients: &Vec<Client>) -> ChannelOutput {
    ChannelOutput {
        name: chan.channel_name.clone(),
        children: Vec::new(),
        cid: chan.cid,
        pid: chan.pid,
        clients: get_clients_by_cid(&clients, &chan.cid),
    }
}

fn get_clients_by_cid(clients: &Vec<Client>, cid: &u32) -> Vec<Client> {
    clients
        .iter()
        .filter(|c| cid == &c.cid)
        .map(|c| c.clone())
        .collect()
}

impl From<&HashMap<String, String>> for Channel {
    fn from(map: &HashMap<String, String>) -> Self {
        let mut channel = Channel::default();
        channel.channel_name = "not found".into();
        for (k, v) in map.iter() {
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
