#[macro_use]
pub mod model;
pub mod access;
pub mod error;
pub mod client;

pub mod live;

pub mod prelude {
    pub use serde::{Serialize, Deserialize, de::DeserializeOwned};
    pub use concat_string::concat_string;
    pub use serde_json::{Value as JsonValue, json};
    pub use serde_repr::{Serialize_repr, Deserialize_repr};
    pub use crate::model::*;
}
