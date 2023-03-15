mod event;
mod message;
mod action;
mod commands;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate serde;

use futures::{SinkExt, StreamExt};
use serde_json::json;
use tokio::join;
use tokio_tungstenite::connect_async;
use tungstenite::http::Uri;
use tungstenite::Message;
use command_macro::command;
use crate::event::Event;

/// Send actions to remote server.
async fn action_sender_channel(mut rx: tokio::sync::mpsc::Receiver<String>) -> Result<(), tungstenite::Error> {
    let (mut socket, _) = connect_async(
        "ws://localhost:8080/api".parse::<Uri>().expect("Can't connect to case count URL"),
    ).await?;

    while let Some(action) = rx.recv().await {
        socket.send(tungstenite::Message::Text(action)).await?;
    }

    rx.close();

    Ok(())
}

async fn event_channel(tx: tokio::sync::mpsc::Sender<Event>) -> Result<(), tungstenite::Error> {
    let (mut socket, _) = connect_async(
        "ws://localhost:8080/event".parse::<Uri>().expect("Can't connect to case count URL"),
    ).await?;
    while let Some(message) = socket.next().await {
        let message = message.expect("Error occur while parsing message");
        let event = serde_json::from_str::<Event>(message.to_text().expect("Message payload is not a valid JSON")).expect("Message payload is not a valid JSON");
        tx.send(event).await.expect("Cannot send to event channel");
    }

    socket.close(None).await
}

async fn process_messages(mut rx: tokio::sync::mpsc::Receiver<Event>, action: tokio::sync::mpsc::Sender<String>) {
    while let Some(event) = rx.recv().await {
        match event {
            Event::Message { raw_message, group_id, .. } => {
                if let Some(group_id) = group_id {

                    raw_message.split_once(' ');
                    action.send(json!({"action": "send_msg", "params": {"group_id": group_id, "message": &raw_message}}).to_string()).await.expect("Cannot send action");
                }
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (event_tx, mut event_rx) = tokio::sync::mpsc::channel(4);

    let (action_tx, mut action_rx) = tokio::sync::mpsc::channel(4);

    let event_channel_handle = tokio::spawn(event_channel(event_tx));
    let process_channel_handle = tokio::spawn(process_messages(event_rx, action_tx.clone()));
    let action_channel_handle = tokio::spawn(action_sender_channel(action_rx));


    join!(event_channel_handle, process_channel_handle, action_channel_handle);
    Ok(())
}

mod tests {
    use std::str::FromStr;
    use std::string::ParseError;
    use command_macro::command;
    use crate::message::Message;

    impl FromStr for Message {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            todo!()
        }
    }

    #[test]
    pub fn test() {
        command!(PingCommand "/ping");

        PingCommand::from_str("ggg");
    }
}