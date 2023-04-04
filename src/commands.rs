use reqwest::{Client, IntoUrl};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;
use serde_json::Value;
use itertools::Itertools;

lazy_static! {
    static ref CLIENT: Client = build_client();
}

fn build_client() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:107.0) Gecko/20100101 Firefox/107.0"));
    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
}

fn ping_command(server: &str) {

}

async fn get_json<T: IntoUrl, D: for<'de> serde::Deserialize<'de>>(url: T) -> reqwest::Result<D> {
    CLIENT.get(url)
        .send()
        .await?
        .json()
        .await
}

pub mod bilibili {
    use futures::future::join_all;
    use itertools::Itertools;
    use crate::commands::get_json;

    #[derive(Debug, Deserialize)]
    pub struct BilibiliResult<T> {
        code: i32,
        message: String,
        data: T,
    }

    #[derive(Error, Debug)]
    pub enum BilibiliError {
        #[error("General error {code}: {message}")]
        General { code: i32, message: String }
    }

    impl<T> From<BilibiliResult<T>> for Result<T, BilibiliError> {
        fn from(value: BilibiliResult<T>) -> Self {
            if value.code == 0 {
                Ok(value.data)
            } else {
                Err(BilibiliError::General { code: value.code, message: value.message })
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct LiveRoomInfo {
        pub uid: u64,
        pub room_id: u64,
        pub title: String,
        pub online: i32,
        pub live_status: i32,
    }

    pub async fn get_live_room_info(id: u64) -> anyhow::Result<LiveRoomInfo> {
        Ok(Result::<_, BilibiliError>::from(
            get_json::<_, BilibiliResult<_>>(format!("https://api.live.bilibili.com/room/v1/Room/get_info?id={}", id))
                .await?
            )?
        )
    }

    #[derive(Debug, Deserialize)]
    pub struct UserInfo {
        #[serde(rename = "mid")]
        pub id: u64,

        pub name: String
    }

    pub async fn get_user_info(id: u64) -> anyhow::Result<UserInfo> {
        Ok(Result::<_, BilibiliError>::from(
            get_json::<_, BilibiliResult<_>>(format!("https://api.bilibili.com/x/space/acc/info?mid={}", id))
                .await?
            )?
        )
    }

    pub async fn streamer_command() -> anyhow::Result<String> {
        let mut any_online = false;
        let mut s = String::new();

        let lives = [6521792u64, 1229190, 5990721, 6813025, 4773124, 23433811, 1628068, 23072941, 3990723, 3995925, 22423059, 478786, 590146, 4367063, 1858917, 23133400, 3758121];

        join_all(join_all(lives.iter().map(|x| get_live_room_info(*x)))
            .await

            .into_iter()
            .filter_ok(|x| x.live_status == 1)
            .map(|x| async {
                let x = x?;
                let uid = x.uid;
                Result::<_, anyhow::Error>::Ok((x, get_user_info(uid).await?))
            })
        )
            .await
            .into_iter()
            .try_for_each(|x| {
                use std::fmt::Write;

                let (room_info, user_info) = x?;
                writeln!(&mut s, "你喜爱的主播 {} 正在直播: {}！https://live.bilibili.com/{}", user_info.name, room_info.title, room_info.room_id).unwrap();
                any_online = true;
                Result::<_, anyhow::Error>::Ok(())
            })?;

        if any_online {
            Ok(s)
        } else {
            Ok("你喜爱的主播们都不在直播哦O(∩_∩)O".to_string())
        }
    }
}

