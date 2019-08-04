use crate::err::Error;
use crate::util;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use telnet::Telnet;

#[derive(Default, Clone)]
pub struct ServerInfo {
    pub virtualserver_port: u32,
    pub virtualserver_unique_identifier: String,
    pub virtualserver_name: String,
}

impl ServerInfo {

    pub fn get(conn: &mut Telnet) -> Result<ServerInfo, Error> {
        conn.write(b"serverinfo\r\n")?;
        thread::sleep(Duration::from_millis(100));
        let event = conn.read()?;

        if let Some(maps) = util::telnet_event_to_hashmap(&event) {
            let infos: Vec<ServerInfo> = maps.iter().map(|map| ServerInfo::from(map)).collect();
            if let Some(info) = infos.get(0) {
                Ok(info.clone())
            } else { Err(Error::NoneError) }
        } else {
            Err(Error::NoneError)
        }
    }
}

impl From<&HashMap<String, String>> for ServerInfo {
    fn from(map: &HashMap<String, String>) -> Self {
        let mut server_info = ServerInfo::default();
        for (k, v) in map.iter() {
            match k.as_str() {
                "virtualserver_port" =>  server_info.virtualserver_port = v.parse().unwrap_or(9987),
                "virtualserver_unique_identifier" => server_info.virtualserver_unique_identifier = v.to_owned(),
                "virtualserver_name" => server_info.virtualserver_name = v.to_owned(),
                _ => {},
            }
        }
        server_info
    }
}