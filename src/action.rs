use tokio::sync::mpsc::Sender;

pub struct Actor {
    sender: Sender<String>
}

impl Actor {
    pub fn send_message(&self) {

    }
}

pub struct SendMessage {

}

pub struct SetEssenceMsg {
    message_id: i32
}