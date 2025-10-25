use serde::{Serialize, Deserialize, de::DeserializeOwned};

#[macro_export]
macro_rules! name {
    () => {
        include!("name")
    };
}

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
            BizKind::Common { .. } => concat!("https://api.", name!(), ".com"),
            BizKind::Live => concat!("https://api.live.", name!(), ".com"),
            BizKind::LegacyDynamic => concat!("https://api.vc.", name!(), ".com"),
        }
    }

    pub fn referer(&self) -> &'static str {
        match self {
            BizKind::Common { from } => match from {
                None => concat!("https://www.", name!(), ".com/"),
                Some(CommonFrom::LegacyDynamic) => concat!("https://t.", name!(), ".com/"),
            },
            BizKind::Live => concat!("https://live.", name!(), ".com/"),
            BizKind::LegacyDynamic => concat!("https://t.", name!(), ".com/"),
        }
    }
}

pub trait RestApi: Serialize {
    const BIZ: BizKind;
    const METHOD: RestApiRequestMethod;
    const PATH: &'static str;
    const DEFAULT: Option<&'static str> = None;
    const WBI: bool = false;
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
