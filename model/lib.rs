use serde::{Serialize, Deserialize, de::DeserializeOwned};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BizKind {
    Common { from: Option<CommonFrom> },
    Live,
    LegacyDynamic,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommonFrom {
    LegacyDynamic,
}

impl BizKind {
    pub fn host(&self) -> &'static str {
        match self {
            BizKind::Common { .. } => "https://api.bilibili.com",
            BizKind::Live => "https://api.live.bilibili.com",
            BizKind::LegacyDynamic => "https://api.vc.bilibili.com",
        }
    }

    pub fn referer(&self) -> &'static str {
        match self {
            BizKind::Common { from } => match from {
                None => "https://www.bilibili.com/",
                Some(CommonFrom::LegacyDynamic) => "https://t.bilibili.com/",
            },
            BizKind::Live => "https://live.bilibili.com/",
            BizKind::LegacyDynamic => "https://t.bilibili.com/",
        }
    }
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
    pub data: Data, // Option<Data> ?
    pub message: String,
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
    pub use serde::{self, Serialize, Deserialize, de::DeserializeOwned};
    pub use serde_json::{self, Value as JsonValue, json};
    pub use serde_repr::{Serialize_repr, Deserialize_repr};
    pub use serde_enum_str::{Serialize_enum_str, Deserialize_enum_str};
    pub use foundations::concat_string;
}
