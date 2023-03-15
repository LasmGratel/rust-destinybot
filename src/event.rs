use clap::builder::Str;
use crate::message::Message;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "post_type", rename_all = "snake_case")]
pub enum Event {
    Message {
        message_type: MessageType,
        sub_type: MessageSubtype,
        message_id: i32,
        user_id: u64,
        message: Vec<Message>,
        raw_message: String,
        font: i32,
        sender: MessageSender,

        group_id: Option<u64>,
    },
    Request,
    Notice,
    MetaEvent
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageSender {
    user_id: u64,
    nickname: String,
    sex: String,
    age: i32,

    card: Option<String>,
    area: Option<String>,
    level: Option<String>,
    role: Option<MemberRole>,
    title: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemberRole {
    Owner,
    Admin,
    Member
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Private, Group
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageSubtype {
    /// 好友
    Friend,

    /// 群聊
    Normal,

    /// 匿名
    Anonymous,

    /// 群中自身发送
    GroupSelf,

    /// 群临时会话
    Group,

    /// 系统提示
    Notice
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventPayload {
    pub time: u64,
    pub self_id: u64,
    pub post_type: String
}