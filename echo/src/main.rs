use std::io;
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
}

fn main() {
    let mut lines = io::stdin().lines();
    let mut stdout = std::io::stdout().lock();

    let initialization_message = lines.next().unwrap().unwrap();
    let init_message:Message = serde_json::from_str(&initialization_message).unwrap();
    println!("{:?}", init_message);
}
