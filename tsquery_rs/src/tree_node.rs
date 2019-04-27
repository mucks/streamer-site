use crate::channel::Channel;
use crate::client::Client;
use telnet::Telnet;

#[derive(Serialize, Debug, Clone)]
pub struct TreeNode {
    pub name: String,
    pub is_client: bool,
    pub children: Vec<TreeNode>,
    pub cid: u32,
    pub pid: u32,
}

impl From<&Channel> for TreeNode {
    fn from(chan: &Channel) -> Self {
        TreeNode {
            name: chan.channel_name.clone(),
            is_client: false,
            children: Vec::new(),
            cid: chan.cid,
            pid: chan.pid,
        }
    }
}

impl From<&Client> for TreeNode {
    fn from(client: &Client) -> Self {
        TreeNode {
            name: client.client_nickname.clone(),
            is_client: true,
            children: Vec::new(),
            cid: client.cid,
            pid: 0,
        }
    }
}

impl TreeNode {
    pub fn get_all(mut conn: &mut Telnet) -> Vec<TreeNode> {
        let clients = crate::client::Client::get_all(&mut conn);
        let channels = crate::channel::Channel::get_all(&mut conn);
        let mut nodes = Vec::new();

        recursive_add_channels(&mut nodes, &channels, &0);
        recursive_add_clients(&mut nodes, &clients);

        nodes
            .iter()
            .filter(|out| out.pid == 0)
            .map(|out| out.clone())
            .collect()
    }
}

fn recursive_add_channels(nodes: &mut Vec<TreeNode>, channels: &Vec<Channel>, node_cid: &u32) {
    for chan in channels {
        if &chan.pid == node_cid {
            nodes.push(TreeNode::from(chan));
        }
    }
    for node in nodes {
        recursive_add_channels(&mut node.children, &channels, &node.cid)
    }
}
fn recursive_add_clients(nodes: &mut Vec<TreeNode>, clients: &Vec<Client>) {
    for node in nodes {
        if !node.is_client {
            for client in &get_clients_by_cid(&clients, &node.cid) {
                node.children.push(TreeNode::from(client));
            }
            recursive_add_clients(&mut node.children, &clients);
        }
    }
}
fn get_clients_by_cid(clients: &Vec<Client>, cid: &u32) -> Vec<Client> {
    clients
        .iter()
        .filter(|c| cid == &c.cid)
        .map(|c| c.clone())
        .collect()
}
