use serde_json::json;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
pub struct Actor {
    sender: Sender<String>
}

impl Actor {
    pub async fn send_group_message(&self, group_id: u64, message: &str) -> Result<(), SendError<String>> {
        self.sender.send(json!({"action": "send_msg", "params": {"group_id": group_id, "message": &message}}).to_string()).await
    }
}

pub struct SendMessage {

}

pub struct SetEssenceMsg {
    message_id: i32
}