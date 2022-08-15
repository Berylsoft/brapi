use serde::{Serialize, Deserialize, de::DeserializeOwned};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BizKind {
    Common { from_page: CommonFromPageKind },
    Live,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommonFromPageKind {
    Home,
    Dynamic,
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
    PostForm,
    PostJson,
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
        BizKind::Common { .. } => "https://api.bilibili.com",
        BizKind::Live => "https://api.live.bilibili.com",
    }
}

pub const fn referer(biz: BizKind) -> &'static str {
    match biz {
        BizKind::Common { from_page } => match from_page {
            CommonFromPageKind::Home => "https://www.bilibili.com/",
            CommonFromPageKind::Dynamic => "https://t.bilibili.com/",
        },
        BizKind::Live => "https://live.bilibili.com/",
    }
}

#[macro_export]
macro_rules! const_json_value_fn_impl {
    ($name: ident, $($json:tt)+) => {
        pub fn $name() -> serde_json::Value {
            serde_json::json!($($json)+)
        }
    };
}

pub mod prelude {
    pub use serde::{Serialize, Deserialize, de::DeserializeOwned};
    pub use concat_string::concat_string;
    pub use serde_json::{Value as JsonValue, json};
    pub use serde_repr::{Serialize_repr, Deserialize_repr};
}
