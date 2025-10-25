use crate::{error::{RestApiError, RestApiResult}, wbi_api::{BasicInfo, WbiImg}};

const KEY_LUT: [u8; 32] = hex_literal::hex!("2e2f1202350817200f320a1f3a032d231b2b053121092a131d1c0e270c26290d");

fn split_key(url: &str) -> Option<&str> {
    let (_, s) = url.rsplit_once("/")?;
    let (s, _) = s.rsplit_once(".")?;
    Some(s)
}

fn get_key(basic_info: BasicInfo) -> RestApiResult<[u8; 32]> {
    let BasicInfo { wbi_img: WbiImg { img_url, sub_url } } = basic_info;
    let img_url_bytes = split_key(&img_url).ok_or(RestApiError::ParseWbiImg)?.as_bytes();
    let sub_url_bytes = split_key(&sub_url).ok_or(RestApiError::ParseWbiImg)?.as_bytes();
    let key = KEY_LUT.map(|n| {
        if n < 32 {
            img_url_bytes[n as usize]
        } else {
            sub_url_bytes[(n - 32) as usize]
        }
    });
    Ok(key)
}

fn sign(orig_params: String, key: &[u8; 32], ts: u64) -> RestApiResult<String> {
    // assume all desered strings have no special chars
    let mut deser_params: Vec<(String, String)> = serde_urlencoded::from_str(&orig_params)?;
    deser_params.push(("wts".to_owned(), ts.to_string()));
    deser_params.sort_by(|a, b| a.0.cmp(&b.0));
    let tosign_params = serde_urlencoded::to_string(deser_params)?;
    let mut md5_ctx = md5::Context::new();
    md5_ctx.consume(&tosign_params);
    md5_ctx.consume(key);
    let md5 = md5_ctx.finalize();
    Ok(format!("{tosign_params}&w_rid={md5:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_key() {
        assert_eq!(
            split_key("https://i0.hdslb.com/bfs/wbi/7cd084941338484aae1ad9425b84077c.png").unwrap(),
            "7cd084941338484aae1ad9425b84077c",
        );
    }

    #[test]
    fn test_get_key() {
        let basic_info = BasicInfo { wbi_img: WbiImg {
            img_url: "https://i0.hdslb.com/bfs/wbi/7cd084941338484aae1ad9425b84077c.png".to_owned(),
            sub_url: "https://i0.hdslb.com/bfs/wbi/4932caff0ff746eab6f01bf08b70ac45.png".to_owned(),
        } };
        assert_eq!(
            get_key(basic_info).unwrap(),
            *b"ea1db124af3c7062474693fa704f4ff8",
        );
    }

    #[test]
    fn test_sign() {
        let orig_params = [
            ("foo", "114"),
            ("bar", "514"),
            ("zab", "1919810"),
        ];
        assert_eq!(
            sign(
                serde_urlencoded::to_string(orig_params).unwrap(),
                b"ea1db124af3c7062474693fa704f4ff8",
                1702204169
            ).unwrap(),
            "bar=514&foo=114&wts=1702204169&zab=1919810&w_rid=8f6f2b5b3d485fe1886cec6a0be8c5d4".to_owned(),
        )
    }
}
