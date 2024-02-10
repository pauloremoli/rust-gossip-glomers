use anyhow::bail;
use rust_dist_sys_challenge::*;
use serde::{Deserialize, Serialize};
use std::io::StdoutLock;
use ulid::Ulid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {},
    Generate {},
    GenerateOk {
        id: String,
    },
}

pub struct UniqueNode {
    pub id: usize,
}

impl Node<Payload> for UniqueNode {
    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Init { .. } => {
                Self::reply(input, output, Payload::InitOk {}, self.id)?;
                self.id += 1;
            }
            Payload::InitOk { .. } => {
                self.id += 1;
                bail!("Received InitOk");
            }
            Payload::Generate {} => {
                Self::reply(
                    input,
                    output,
                    Payload::GenerateOk {
                        id: Ulid::new().to_string(),
                    },
                    self.id,
                )?;
                self.id += 1;
            }
            Payload::GenerateOk { .. } => {}
        }

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    main_loop(UniqueNode { id: 1 })
}
