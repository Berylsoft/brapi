use crate::{error::{RestApiError, RestApiResult}, wbi_api::{BasicInfo, WbiImg}};

const KEY_LUT: [u8; 32] = hex_literal::hex!("2e2f1202350817200f320a1f3a032d231b2b053121092a131d1c0e270c26290d");

fn split_key(url: &str) -> Option<&str> {
    let (_, s) = url.rsplit_once("/")?;
    let (s, _) = s.rsplit_once(".")?;
    Some(s)
}

fn get_key(basic_info: BasicInfo) -> RestApiResult<[u8; 32]> {
    let BasicInfo { wbi_img: WbiImg { img_url, sub_url } } = basic_info;
    let mut full = String::with_capacity(64); // TODO remapping to avoid this buffer
    full.push_str(split_key(&img_url).ok_or(RestApiError::ParseWbiImg)?);
    full.push_str(split_key(&sub_url).ok_or(RestApiError::ParseWbiImg)?);
    let full_bytes = full.as_bytes();
    let key = KEY_LUT.map(|n| full_bytes[n as usize]);
    Ok(key)
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
}
