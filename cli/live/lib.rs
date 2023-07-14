use brapi_client::client::Client;
use brapi_live as api;

fn now_signed() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().try_into().unwrap()
}

#[derive(argh::FromArgs)]
#[argh(subcommand)]
pub enum Request {
    Danmaku(Danmaku),
}

/// send a common danmaku to specified room
#[derive(argh::FromArgs)]
#[argh(subcommand, name = "send-dm")]
pub struct Danmaku {
    /// target roomid (no short id)
    #[argh(option, short = 'r')]
    roomid: u32,
    /// danmaku content
    #[argh(option, short = 't')]
    text: String,
    /// customize the rand value
    #[argh(option)]
    rand: Option<i64>,
    /// emoji danmaku
    #[argh(switch)]
    emoji: bool,
}

impl Danmaku {
    pub fn conv(self) -> api::interact::SendDanmaku {
        let Danmaku { roomid, text: msg, rand: rnd, emoji } = self;
        let rnd = rnd.unwrap_or_else(now_signed);
        api::interact::SendDanmaku::new(roomid, msg, rnd, emoji)
    }
}

impl Request {
    pub async fn call(self, client: Client) {
        match self {
            Request::Danmaku(payload) => println!("{:?}", client.call(&payload.conv()).await),
        }
    }
}
