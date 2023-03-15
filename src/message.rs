#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum Message {
    Text { text: String },
    At { qq: String },
    Image { file: String, url: Option<String> },
    Record { file: String, url: Option<String> },
    Music {
        /// QQ, 163, xm, custom
        #[serde(rename = "type")]
        music_type: String,
        id: Option<u64>,
        audio: Option<String>,
        url: Option<String>,
        title: Option<String>
    },
    Video { file: String, url: Option<String> },
    File { file_id: String },
    Poke {
        poke_type: String,
        id: String,
    },
    Location { lat: f64, lon: f64, title: Option<String>, content: Option<String> },
    Reply { id: String },
    Forward { id: u64 }
}