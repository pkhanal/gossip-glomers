use std::io::{self, Write, StdoutLock};
use anyhow::{Result, Context};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    src: String,
    dest: String,
    body: MessageBody,
}

#[derive(Serialize, Deserialize, Debug)]
struct MessageBody {
    msg_id: Option<usize>,
    in_reply_to: Option<usize>,

    #[serde(flatten)]
    payload: TypeAwarePayload,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum TypeAwarePayload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
}

struct Node {
    counter: usize,
}

impl Node {
    fn handle(&mut self, message: Message, out: &mut StdoutLock) -> anyhow::Result<()> {
        match message.body.payload {
            TypeAwarePayload::Init { .. } => {
                let reply = Message {
                    src: message.dest,
                    dest: message.src,
                    body: MessageBody {
                        msg_id: Some(self.counter),
                        in_reply_to: message.body.msg_id,
                        payload: TypeAwarePayload::InitOk,
                    },
                };
        
                let output: String = serde_json::to_string(&reply).context("Failed to serialize message")?;
                out.write_all(output.as_bytes()).context("Failed to write to stdout")?;
                out.write_all(b"\n").context("Failed to write to stdout")?;
                self.counter += 1;
            }
            TypeAwarePayload::InitOk => { }
            TypeAwarePayload::Echo { echo } => {
                let reply = Message {
                    src: message.dest,
                    dest: message.src,
                    body: MessageBody {
                        msg_id: Some(self.counter),
                        in_reply_to: message.body.msg_id,
                        payload: TypeAwarePayload::EchoOk { echo },
                    },
                };
        
                let output: String = serde_json::to_string(&reply).context("Failed to serialize message")?;
                out.write_all(output.as_bytes()).context("Failed to write to stdout")?;
                out.write_all(b"\n").context("Failed to write to stdout")?;
                self.counter += 1;
            }
            TypeAwarePayload::EchoOk { .. } => { }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut lines = io::stdin().lines();
    let mut stdout = std::io::stdout().lock();

    let initialization_message = lines.next().unwrap().unwrap();
    let init_message:Message = serde_json::from_str(&initialization_message).unwrap();

    if !matches!(init_message.body.payload, TypeAwarePayload::Init { .. }) {
        panic!("First message should be init message");
    }

    let mut node = Node {
        counter: 0,
    };

    node.handle(init_message, &mut stdout).context("Failed to handle message")?;

    for line in lines {
        let raw_message = line.context("Failed to read line")?;
        let message:Message = serde_json::from_str(&raw_message).context("Failed to parse message")?;
        node.handle(message, &mut stdout).context("Failed to handle message")?;
    }
    
    Ok(())
}
