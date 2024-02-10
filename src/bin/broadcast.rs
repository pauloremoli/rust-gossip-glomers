use anyhow::bail;
use gossip_glomers::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::StdoutLock};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {},
    Broadcast {
        message: u32,
    },
    BroadcastOk {},
    Read {},
    ReadOk {
        messages: Vec<u32>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk {},
}

pub struct BroadcastNode {
    pub id: usize,
    pub received_messages: Vec<u32>,
    pub topology: HashMap<String, Vec<String>>,
}

impl Node<Payload> for BroadcastNode {
    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Init { .. } => {
                self.id += 1;
                Self::reply(input, output, Payload::InitOk {}, self.id)?;
            }
            Payload::InitOk { .. } => {
                self.id += 1;
                bail!("Received InitOk");
            }
            Payload::Broadcast { message } => {
                self.received_messages.push(message);
                self.id += 1;
                Self::reply(input, output, Payload::BroadcastOk {}, self.id)?;
            }
            Payload::BroadcastOk {} => {}
            Payload::Read {} => {
                self.id += 1;
                Self::reply(
                    input,
                    output,
                    Payload::ReadOk {
                        messages: self.received_messages.clone(),
                    },
                    self.id,
                )?;
            }
            Payload::ReadOk { .. } => {}
            Payload::Topology { ref topology } => {
                self.topology = topology.clone();
                self.id += 1;
                Self::reply(input, output, Payload::TopologyOk {}, self.id)?;
            }
            Payload::TopologyOk {} => {}
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    main_loop(BroadcastNode {
        id: 0,
        received_messages: Vec::new(),
        topology: HashMap::new(),
    })
}
