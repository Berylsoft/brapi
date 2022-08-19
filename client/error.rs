pub use hyper::{Error as HttpError, Result as HttpResult};

#[derive(Debug)]
pub enum RestApiFailureCode {
    FromHttp(u16),
    FromApi { code: i32, message: String },
}

macro_rules! error_conv_impl {
    {$name:ident {$($extra:tt)+} conv {$($variant:ident => $error:ty),*,}} => {
        #[derive(Debug)]
        pub enum $name {
            $($variant($error),)*
            $($extra)+
        }

        $(
            impl From<$error> for $name {
                fn from(err: $error) -> $name {
                    <$name>::$variant(err)
                }
            }
        )*
    };
}

error_conv_impl! {
    RestApiError
    {
        Failure {
            code: RestApiFailureCode,
            payload: String,
            rate_limited: bool,
        },
        PostWithoutAccess,
    }
    conv {
        Network        => HttpError,
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
