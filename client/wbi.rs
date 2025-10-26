use crate::{error::{RestApiError, RestApiResult}, wbi_api::{BasicInfo, WbiImg}};

const KEY_LUT: [u8; 32] = hex_literal::hex!("2e2f1202350817200f320a1f3a032d231b2b053121092a131d1c0e270c26290d");

fn split_key(url: &str) -> Option<&str> {
    let (_, s) = url.rsplit_once("/")?;
    let (s, _) = s.rsplit_once(".")?;
    Some(s)
}

pub fn get_key(basic_info: BasicInfo) -> RestApiResult<[u8; 32]> {
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

pub fn sign(orig_params: String, key: &[u8; 32], ts: u64) -> RestApiResult<String> {
    // assume all desered strings have no special chars
    let mut deser_params: Vec<(&str, &str)> = serde_urlencoded::from_str(&orig_params)?;
    let wts = ts.to_string();
    deser_params.push(("wts", wts.as_str()));
    deser_params.sort_by(|a, b| a.0.cmp(b.0));
    let tosign_params = serde_urlencoded::to_string(&deser_params)?;
    let mut md5_ctx = md5::Context::new();
    md5_ctx.consume(&tosign_params);
    md5_ctx.consume(key);
    let w_rid = format!("{:?}", md5_ctx.finalize());
    let mut final_deser_params = Vec::with_capacity(deser_params.len() + 1);
    let mut wts_slot = None;
    for item in deser_params {
        if item.0 == "wts" {
            assert!(matches!(wts_slot.replace(item), None));
        } else {
            final_deser_params.push(item);
        }
    }
    final_deser_params.push(("w_rid", w_rid.as_str()));
    final_deser_params.push(wts_slot.unwrap());
    let final_params = serde_urlencoded::to_string(final_deser_params)?;
    Ok(final_params)
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
            "bar=514&foo=114&zab=1919810&w_rid=8f6f2b5b3d485fe1886cec6a0be8c5d4&wts=1702204169".to_owned(),
        );
        assert_eq!(
            sign(
                "id=5440&type=0&web_location=444.8".to_owned(),
                b"ea1db124af3c7062474693fa704f4ff8",
                1761421538
            ).unwrap(),
            "id=5440&type=0&web_location=444.8&w_rid=3edf41f048e625e8a612a56c36c5bb29&wts=1761421538".to_owned(),
        );
    }
}
