use anyhow::{bail, Context};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::{StdoutLock, Write};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message<Payload> {
    pub src: String,
    pub dest: String,
    pub body: Body<Payload>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Body<Payload> {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}

pub trait Node<Payload> {
    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()>;

    fn reply(input: Message<Payload>, output: &mut StdoutLock, payload: Payload, msg_id: usize) -> anyhow::Result<()> 
    where Message<Payload>: Serialize
    {
        let reply = Message {
            src: input.dest,
            dest: input.src,
            body: Body {
                msg_id: Some(msg_id),
                in_reply_to: input.body.msg_id,
                payload,
            },
        };

        serde_json::to_writer(&mut *output, &reply)
            .context("Failed to serialize response to Echo")?;

        output.write_all(b"\n").context("write line break")?;
        Ok(())
    }
}

pub fn main_loop<S, Payload>(mut state: S) -> anyhow::Result<()>
where
    S: Node<Payload>,
    Payload: DeserializeOwned,
{
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();

    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message<Payload>>();
    for input in inputs {
        let input = input.context("Mailstrom input from STDIN could not be derialized")?;
        state
            .step(input, &mut stdout)
            .context("Node step function failed")?;
    }

    Ok(())
}
