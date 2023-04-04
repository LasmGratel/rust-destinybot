use std::str::FromStr;
use std::string::ParseError;
use command_macro::command;
use crate::commands::bilibili::{get_live_room_info, get_user_info, streamer_command};

#[test]
pub fn test() {
    command!(PingCommand "/ping" |x, y| { println!("{}, {}", x, y); } String, i32);

    PingCommand::from_str("ggg").expect("TODO: panic message");
}

#[tokio::test]
pub async fn test_room_info() {
    let info = get_live_room_info(213).await.unwrap();
    assert_eq!(info.uid, 67141);

    let info = get_user_info(info.uid).await.unwrap();
    assert_eq!(info.id, 67141);
}