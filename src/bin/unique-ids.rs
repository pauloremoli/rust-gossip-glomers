use anyhow::{bail, Context};
use rust_dist_sys_challenge::*;
use serde::{Deserialize, Serialize};
use std::io::{StdoutLock, Write};
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
    GenerateOk { id: String},
}

pub struct UniqueNode {
    pub id: usize,
}

impl Node<Payload> for UniqueNode {
    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Init { .. } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        msg_id: Some(self.id),
                        in_reply_to: input.body.msg_id,
                        payload: Payload::InitOk {},
                    },
                };

                serde_json::to_writer(&mut *output, &reply)
                    .context("Failed to serialize response to Init")?;
                output.write_all(b"\n").context("write line break")?;
                self.id += 1;
            }
            Payload::InitOk { .. } => {
                bail!("Received InitOk");
            }
            Payload::Generate {} => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        msg_id: Some(self.id),
                        in_reply_to: input.body.msg_id,
                        payload: Payload::GenerateOk { id: Ulid::new().to_string() },
                    },
                };

                serde_json::to_writer(&mut *output, &reply)
                    .context("Failed to serialize response to Init")?;
                output.write_all(b"\n").context("write line break")?;
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
