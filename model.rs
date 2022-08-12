use serde::{Serialize, Deserialize, de::DeserializeOwned};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BizKind {
    Live,
}

pub trait RestApi: Serialize {
    const BIZ: BizKind;
    const METHOD: RestApiRequestMethod;
    const PATH: &'static str;
    const DEFAULT: Option<&'static str>;
    type Response: DeserializeOwned;
}

pub enum RestApiRequestMethod {
    BareGet,
    Get,
    Post { form: bool },
}

#[derive(Deserialize)]
pub struct RestApiResponse<Data> { // Data: DeserializeOwned
    pub code: i32,
    pub data: Data,
    pub message: String,
}

pub const WEB_USER_AGENT: &'static str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.0.0 Safari/537.36";

pub const fn api_host(biz: BizKind) -> &'static str {
    match biz {
        BizKind::Live => "https://api.live.bilibili.com",
    }
}

pub const fn referer(biz: BizKind) -> &'static str {
    match biz {
        BizKind::Live => "https://live.bilibili.com/",
    }
}
