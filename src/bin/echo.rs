use anyhow::bail;
use gossip_glomers::*;
use serde::{Deserialize, Serialize};
use std::io::StdoutLock;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {},
}

pub struct EchoNode {
    pub id: usize,
}

impl Node<Payload> for EchoNode {
    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Init { .. } => {
                Self::reply(input, output, Payload::InitOk {}, self.id)?;
                self.id += 1;
            }
            Payload::InitOk { .. } => {
                bail!("Received InitOk");
            }
            Payload::Echo { ref echo } => {
                let echo = echo.clone();
                Self::reply(input, output, Payload::EchoOk { echo }, self.id)?;
                self.id += 1;
            }
            Payload::EchoOk { .. } => {}
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    main_loop(EchoNode { id: 1 })
}
