use crate::err::Error;
use crate::util;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use telnet::Telnet;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Client {
    pub client_nickname: String,
    pub clid: u32,
    pub cid: u32,
    pub client_type: u32,
    pub client_database_id: u32,
    pub msg: Option<String>,
}

impl Client {
    pub fn get_all(conn: &mut Telnet) -> Result<Vec<Client>, Error> {
        conn.write(b"clientlist\r\n")?;
        thread::sleep(Duration::from_millis(100));
        let event = conn.read()?;
        if let Some(maps) = util::telnet_event_to_hashmap(&event) {
            Ok(maps.iter().map(|map| Client::from(map)).collect())
        } else {
            Err(Error::NoneError)
        }
    }
}

impl From<&HashMap<String, String>> for Client {
    fn from(map: &HashMap<String, String>) -> Client {
        let mut client = Client::default();

        for (k, v) in map {
            if k.as_str() == "client_nickname" {
                client.client_nickname = v.to_owned();
            } else if k.as_str() == "msg" {
                client.msg = Some(v.to_owned());
            } else {
                let mut temp: u32 = 0;
                let field = match k.as_str() {
                    "clid" => &mut client.clid,
                    "cid" => &mut client.cid,
                    "client_type" => &mut client.client_type,
                    "client_database_id" => &mut client.client_database_id,
                    _ => &mut temp,
                };
                *field = v.parse().unwrap_or(0);
            }
        }

        client
    }
}
