pub use hyper::{Error as HyperError, Result as HyperResult};

#[derive(Debug)]
pub enum RestApiFailureCode {
    FromHttp(u16),
    FromApi { code: i32, message: String },
}

foundations::error_enum! {
    #[derive(Debug)]
    pub enum RestApiError {
        Failure {
            code: RestApiFailureCode,
            payload: String,
            rate_limited: bool,
        },
        PostWithoutAccess,
    }
    convert {
        Io             => std::io::Error,
        Hyper          => HyperError,
        ParseString    => std::string::FromUtf8Error,
        ParseStr       => std::str::Utf8Error,
        Parse          => serde_json::Error,
        EncodePostBody => serde_urlencoded::ser::Error,
    }
}

impl std::fmt::Display for RestApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for RestApiError {}

pub type RestApiResult<Data> = Result<Data, RestApiError>;
