use bilibili_restapi_model::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Access {
    pub uid: u64,
    pub key: String,
    pub csrf: String,
}

fn split_into_kv(pair: &str, pat: char) -> Option<(&str, &str)> {
    // ref: https://doc.servo.org/src/cookie/parse.rs.html#108-111
    pair.find(pat).map(|i| (&pair[..i], &pair[(i + 1)..]))
}

const K_UID: &str = "DedeUserID";
const K_KEY: &str = "SESSDATA";
const K_CSRF: &str = "bili_jct";

impl Access {
    pub fn from_cookie<S: AsRef<str>>(cookie: S) -> Option<Access> {
        macro_rules! seat {
            ($name:tt, $ty:ty) => {
                let mut $name: Option<$ty> = None;
            };
        }

        macro_rules! occupy {
            ($name:ident, $value:expr) => {{
                if let Some(_) = $name.replace($value) { return None };
            }};
        }

        seat!(uid, u64);
        seat!(key, String);
        seat!(csrf, String);

        for pair in cookie.as_ref().split(';') {
            let (k, v) = split_into_kv(pair.trim(), '=')?;
            let (k, v) = (k.trim(), v.trim());

            match k {
                K_UID => occupy!(uid, v.parse().ok()?),
                K_KEY => occupy!(key, v.to_owned()),
                K_CSRF => occupy!(csrf, v.to_owned()),
                _ => { },
            }
        }

        Some(Access {
            uid: uid?,
            key: key?,
            csrf: csrf?,
        })
    }

    pub fn as_cookie(&self) -> String {
        concat_string!(
            K_UID, "=", self.uid.to_string(), "; ",
            K_KEY, "=", self.key, "; ",
            K_CSRF, "=", self.csrf
        )
    }
}
