use brapi_model::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Access {
    pub uid: u64,
    pub key: String,
    pub csrf: String,
    pub devid3: String,
    // TODO use Bytes
    pub raw: String,
}

fn split_into_kv(pair: &str, pat: char) -> Option<(&str, &str)> {
    // ref: https://doc.servo.org/src/cookie/parse.rs.html#108-111
    pair.find(pat).map(|i| (&pair[..i], &pair[(i + 1)..]))
}

const K_UID: &str = "DedeUserID";
const K_KEY: &str = "SESSDATA";
const K_CSRF: &str = "bili_jct";
const K_DEVID3: &str = "buvid3";

impl Access {
    pub fn from_raw(raw: String) -> Option<Access> {
        macro_rules! seat_impl {
            ($struct:tt, $input:expr; $($name:ident, $k:pat, $ty:ty;)*; $($rest:tt)*) => {{
                $(let mut $name: Option<$ty> = None;)*

                for pair in $input.split(';') {
                    let (k, v) = split_into_kv(pair.trim(), '=')?;
                    let (k, v) = (k.trim(), v.trim());

                    match k {
                        $($k => { if let Some(_) = $name.replace(v.parse().ok()?) { return None }; })*
                        _ => { },
                    }
                }

                Some($struct {
                    $($name: $name?,)*
                    $($rest)*
                })
            }};
        }

        seat_impl!(
            Access, raw;
            uid, K_UID, u64;
            key, K_KEY, String;
            csrf, K_CSRF, String;
            devid3, K_DEVID3, String;;
            raw,
        )
    }

    // pub fn as_cookie(&self) -> String {
    //     concat_string!(
    //         K_UID, "=", self.uid.to_string(), "; ",
    //         K_KEY, "=", self.key, "; ",
    //         K_CSRF, "=", self.csrf, "; ",
    //         K_DEVID3, "=", self.devid3
    //     )
    // }
}
