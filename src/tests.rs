use std::str::FromStr;
use std::string::ParseError;
use command_macro::command;
use crate::event::MemberRole;
use crate::commands::bilibili::{get_live_room_info, get_user_info, streamer_command};

#[test]
pub fn test() {
}

#[command("/ping", "ping")]
struct PingCommand {
    server: String
}

#[tokio::test]
pub async fn test_room_info() {
    let info = get_live_room_info(213).await.unwrap();
    assert_eq!(info.uid, 67141);

    let info = get_user_info(info.uid).await.unwrap();
    assert_eq!(info.id, 67141);
}