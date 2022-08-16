#![allow(non_camel_case_types)] // for `FormatKind` only but invalid after proc macro of `serde-enum-str`

use bilibili_restapi_model::{*, prelude::*};

#[derive(Clone, Debug, Serialize)]
pub struct GetStreamInfoV1 {
    #[serde(rename = "cid")]
    pub roomid: u32,
    pub qn: i32,
}

impl RestApi for GetStreamInfoV1 {
    const BIZ: BizKind = BizKind::Live;
    const METHOD: RestApiRequestMethod = RestApiRequestMethod::Get;
    const PATH: &'static str = "/room/v1/Room/playUrl";
    const DEFAULT: Option<&'static str> = Some("platform=web");
    type Response = StreamInfoV1;
}

#[derive(Clone, Debug, Deserialize)]
pub struct StreamInfoV1 {
    #[cfg(feature = "unclarified_fields")]
    /// [qn=10000] `4`
    pub current_quality: JsonValue,
    #[cfg(feature = "unclarified_fields")]
    /// [qn=10000] `["4","3"]`
    pub accept_quality: JsonValue,
    pub current_qn: i32,
    #[cfg(feature = "useless_fields")]
    pub quality_description: Vec<QnDescV1>,
    #[serde(rename = "durl")]
    pub urls: Option<Vec<StreamUrlV1>>,
}

#[cfg(feature = "useless_fields")]
#[derive(Clone, Debug, Deserialize)]
pub struct QnDescV1 {
    pub qn: i32,
    pub desc: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StreamUrlV1 {
    pub url: String,
    #[cfg(feature = "unclarified_fields")]
    /// `0`
    pub length: JsonValue,
    #[cfg(feature = "unclarified_fields")]
    /// `1`
    pub order: JsonValue,
    #[cfg(feature = "unclarified_fields")]
    /// `0`
    pub stream_type: JsonValue,
    #[cfg(feature = "unclarified_fields")]
    /// `1`
    pub p2p_type: JsonValue,
}

#[derive(Clone, Debug, Serialize)]
pub struct GetStreamInfoV2 {
    #[serde(rename = "room_id")]
    pub roomid: u32,
    pub qn: i32,
}

impl RestApi for GetStreamInfoV2 {
    const BIZ: BizKind = BizKind::Live;
    const METHOD: RestApiRequestMethod = RestApiRequestMethod::Get;
    const PATH: &'static str = "/xlive/web-room/v2/index/getRoomPlayInfo";
    const DEFAULT: Option<&'static str> = Some("protocol=0,1&format=0,1,2&codec=0,1&platform=web&ptype=8");
    type Response = StreamInfoV2;
}

#[derive(Clone, Debug, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum LiveStatus {
    Off = 0,
    On = 1,
    Idle = 2,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StreamInfoV2 {
    #[serde(rename = "room_id")]
    pub roomid: u32,
    pub short_id: u32,
    pub uid: u64,
    pub is_hidden: bool,
    pub is_locked: bool,
    pub is_portrait: bool,
    pub live_status: LiveStatus,
    /// ts_s (?)
    pub hidden_till: u64,
    /// ts_s (?)
    pub lock_till: u64,
    pub encrypted: bool,
    pub pwd_verified: bool,
    /// ts_s
    pub live_time: u64,
    #[cfg(feature = "unclarified_fields")]
    /// `1`
    pub room_shield: JsonValue,
    #[cfg(feature = "unclarified_fields")]
    /// `[{Number}]`
    pub all_special_types: JsonValue,
    #[serde(rename = "playurl_info")]
    pub urls: Option<StreamUrlsV2>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StreamUrlsV2 {
    #[cfg(feature = "unclarified_fields")]
    /// `"{\"cdn_rate\":10000,\"report_interval_sec\":150}"`
    pub conf_json: JsonValue,
    #[serde(rename = "playurl")]
    pub inner: StreamUrlsV2Inner,
}

#[cfg(feature = "useless_fields")]
#[derive(Clone, Debug, Deserialize)]
pub struct QnDescV2 {
    pub qn: i32,
    pub desc: String,
    pub hdr_desc: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StreamUrlsV2Inner {
    #[cfg(feature = "useless_fields")]
    #[serde(rename = "roomid")]
    pub cid: u32,
    #[cfg(feature = "useless_fields")]
    pub g_qn_desc: Vec<QnDescV2>,
    #[serde(rename = "stream")]
    pub protocols: Vec<StreamUrlsV2Protocol>,
}

#[derive(Clone, Debug, Deserialize_enum_str, PartialEq)]
pub enum ProtocolKind {
    #[serde(rename = "http_stream")]
    HttpStream,
    #[serde(rename = "http_hls")]
    HttpHLS,
    #[serde(other)]
    Unknown(String),
}

#[derive(Clone, Debug, Deserialize)]
pub struct StreamUrlsV2Protocol {
    #[serde(rename = "protocol_name")]
    pub protocol_kind: ProtocolKind,
    #[serde(rename = "format")]
    pub formats: Vec<StreamUrlsV2Format>,
}

#[derive(Clone, Debug, Deserialize_enum_str, PartialEq)]
pub enum FormatKind {
    #[serde(rename = "flv")]
    HTTP_FLV,
    #[serde(rename = "ts")]
    HLS_TS,
    #[serde(rename = "fmp4")]
    HLS_FMP4,
    #[serde(other)]
    Unknown(String),
}

#[derive(Clone, Debug, Deserialize)]
pub struct StreamUrlsV2Format {
    #[serde(rename = "format_name")]
    pub format_kind: FormatKind,
    #[serde(rename = "codec")]
    pub codecs: Vec<StreamUrlsV2Codec>,
}

#[derive(Clone, Debug, Deserialize_enum_str, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CodecKind {
    #[serde(rename = "avc")]
    AVC,
    #[serde(rename = "hevc")]
    HEVC,
    #[serde(other)]
    Unknown(String),
}

#[derive(Clone, Debug, Deserialize)]
pub struct StreamUrlsV2Codec {
    #[serde(rename = "codec_name")]
    pub codec_kind: CodecKind,
    #[serde(flatten)]
    pub urls: StreamUrlV2,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StreamUrlV2 {
    pub current_qn: i32,
    pub accept_qn: Vec<i32>,
    pub base_url: String,
    #[serde(rename = "url_info")]
    pub hosts: Vec<StreamUrlV2Host>,
    #[cfg(feature = "unclarified_fields")]
    /// `null`
    pub hdr_qn: JsonValue,
    #[cfg(feature = "unclarified_fields")]
    /// `0`
    pub dolby_type: JsonValue,
    #[cfg(feature = "unclarified_fields")]
    /// `""`
    pub attr_name: JsonValue,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StreamUrlV2Host {
    pub host: String,
    #[serde(rename = "extra")]
    pub token: String,
    #[cfg(feature = "unclarified_fields")]
    /// `3600`
    pub stream_ttl: JsonValue,
}
#[cfg(test)]
mod tests {
    use super::*;
    use bilibili_restapi_model::prelude::serde_json;

    #[test]
    fn test() {
        let v2 = r#"{"room_id":8016907,"short_id":616,"uid":50329220,"is_hidden":false,"is_locked":false,"is_portrait":false,"live_status":1,"hidden_till":0,"lock_till":0,"encrypted":false,"pwd_verified":true,"live_time":1660634297,"room_shield":1,"all_special_types":[19],"playurl_info":{"conf_json":"{\"cdn_rate\":10000,\"report_interval_sec\":150}","playurl":{"cid":8016907,"g_qn_desc":[{"qn":30000,"desc":"杜比","hdr_desc":""},{"qn":20000,"desc":"4K","hdr_desc":""},{"qn":10000,"desc":"原画","hdr_desc":""},{"qn":400,"desc":"蓝光","hdr_desc":"HDR"},{"qn":250,"desc":"超清","hdr_desc":"HDR"},{"qn":150,"desc":"高清","hdr_desc":""},{"qn":80,"desc":"流畅","hdr_desc":""}],"stream":[{"protocol_name":"http_stream","format":[{"format_name":"flv","codec":[{"codec_name":"avc","current_qn":10000,"accept_qn":[10000,400,250,150],"base_url":"/live-bvc/198135/live_50329220_7780332_bluray.flv?","url_info":[{"host":"https://cn-hblf-ct-01-07.bilivideo.com","extra":"expires=1660667382\u0026len=0\u0026oi=3742159716\u0026pt=web\u0026qn=10000\u0026trid=10009ab0eb3b30ab4c049489899a8170d070\u0026sigparams=cdn,expires,len,oi,pt,qn,trid\u0026cdn=cn-gotcha01\u0026sign=5fda353c79bec4298daeed1d06544c4c\u0026sk=2935686d6cb9146c7a6a6a0b4e120e250342be3df4dc8310261aab0ce9e21e44\u0026p2p_type=1\u0026src=57345\u0026sl=2\u0026free_type=0\u0026sid=cn-hblf-ct-01-07\u0026chash=1\u0026sche=ban\u0026pp=rtmp\u0026machinezone=jd\u0026source=onetier\u0026site=8be15a488474639d0db7e90b31717521\u0026order=1","stream_ttl":3600}],"hdr_qn":null,"dolby_type":0,"attr_name":""},{"codec_name":"he1vc","current_qn":10000,"accept_qn":[10000,400,250],"base_url":"/live-bvc/198135/live_50329220_7780332_prohevc.flv?","url_info":[{"host":"https://cn-hblf-ct-01-15.bilivideo.com","extra":"expires=1660667382\u0026len=0\u0026oi=3742159716\u0026pt=web\u0026qn=10000\u0026trid=10009ab0eb3b30ab4c049489899a8170d070\u0026sigparams=cdn,expires,len,oi,pt,qn,trid\u0026cdn=cn-gotcha01\u0026sign=f9fcc12ff6c8abe0e66291b5cead237d\u0026sk=2935686d6cb9146c7a6a6a0b4e120e250342be3df4dc8310261aab0ce9e21e44\u0026p2p_type=1\u0026src=57345\u0026sl=2\u0026free_type=0\u0026sid=cn-hblf-ct-01-15\u0026chash=1\u0026sche=ban\u0026pp=rtmp\u0026machinezone=jd\u0026source=onetier\u0026site=8be15a488474639d0db7e90b31717521\u0026order=1","stream_ttl":3600}],"hdr_qn":null,"dolby_type":0,"attr_name":""}]}]},{"protocol_name":"http_hls","format":[{"format_name":"ts","codec":[{"codec_name":"avc","current_qn":10000,"accept_qn":[10000,400,250,150],"base_url":"/live-bvc/198135/live_50329220_7780332_bluray.m3u8?","url_info":[{"host":"https://cn-hblf-ct-01-15.bilivideo.com","extra":"expires=1660667382\u0026len=0\u0026oi=3742159716\u0026pt=web\u0026qn=10000\u0026trid=10039ab0eb3b30ab4c049489899a8170d070\u0026sigparams=cdn,expires,len,oi,pt,qn,trid\u0026cdn=cn-gotcha01\u0026sign=8bded16bf5e95a0b93d52c60b59f31c4\u0026sk=2935686d6cb9146c7a6a6a0b4e120e250342be3df4dc8310261aab0ce9e21e44\u0026p2p_type=1\u0026src=57345\u0026sl=2\u0026free_type=0\u0026sid=cn-hblf-ct-01-15\u0026chash=0\u0026sche=ban\u0026pp=rtmp\u0026machinezone=jd\u0026source=onetier\u0026site=8be15a488474639d0db7e90b31717521\u0026order=1","stream_ttl":3600}],"hdr_qn":null,"dolby_type":0,"attr_name":""}]},{"format_name":"fmp4","codec":[{"codec_name":"avc","current_qn":10000,"accept_qn":[10000,400,250,150],"base_url":"/live-bvc/198135/live_50329220_7780332_bluray/index.m3u8?","url_info":[{"host":"https://d1--cn-gotcha208.bilivideo.com","extra":"expires=1660667382\u0026len=0\u0026oi=3742159716\u0026pt=web\u0026qn=10000\u0026trid=10079ab0eb3b30ab4c049489899a8170d070\u0026sigparams=cdn,expires,len,oi,pt,qn,trid\u0026cdn=cn-gotcha208\u0026sign=e61588344ed2833cf9df87efb5ef1fb2\u0026sk=c9c6154426932efa80d25af02e87a3bd\u0026p2p_type=1\u0026src=57345\u0026sl=2\u0026free_type=0\u0026pp=rtmp\u0026machinezone=jd\u0026source=onetier\u0026site=8be15a488474639d0db7e90b31717521\u0026order=1","stream_ttl":3600}],"hdr_qn":null,"dolby_type":0,"attr_name":""},{"codec_name":"hevc","current_qn":10000,"accept_qn":[10000,400,250],"base_url":"/live-bvc/198135/live_50329220_7780332_prohevc/index.m3u8?","url_info":[{"host":"https://d1--cn-gotcha208.bilivideo.com","extra":"expires=1660667382\u0026len=0\u0026oi=3742159716\u0026pt=web\u0026qn=10000\u0026trid=10079ab0eb3b30ab4c049489899a8170d070\u0026sigparams=cdn,expires,len,oi,pt,qn,trid\u0026cdn=cn-gotcha208\u0026sign=2f6b4c34f1e331868048351a51058ec2\u0026sk=c9c6154426932efa80d25af02e87a3bd\u0026p2p_type=1\u0026src=57345\u0026sl=2\u0026free_type=0\u0026pp=rtmp\u0026machinezone=jd\u0026source=onetier\u0026site=8be15a488474639d0db7e90b31717521\u0026order=1","stream_ttl":3600}],"hdr_qn":null,"dolby_type":0,"attr_name":""}]}]}],"p2p_data":{"p2p":true,"p2p_type":1,"m_p2p":false,"m_servers":null},"dolby_qn":null}}}"#;
        let v1 = r#"{"current_quality":4,"accept_quality":["4","3"],"current_qn":10000,"quality_description":[{"qn":10000,"desc":"原画"},{"qn":400,"desc":"蓝光"},{"qn":250,"desc":"超清"},{"qn":150,"desc":"高清"}],"durl":[{"url":"https://d0--cn-gotcha01.bilivideo.com/live-bvc/340487/live_50329220_7780332_bluray.flv?cdn=cn-gotcha01\u0026expires=1660667665\u0026len=0\u0026oi=3742159716\u0026pt=web\u0026qn=10000\u0026trid=1000b90fa53d5f3d4dc6be2b87cb3a04bc69\u0026sigparams=cdn,expires,len,oi,pt,qn,trid\u0026sign=70c2bf91b03b1fbc506a57ef94d572ab\u0026ptype=0\u0026src=57345\u0026sl=2\u0026source=one\u0026sche=ban\u0026sk=2935686d6cb9146c7a6a6a0b4e120e250342be3df4dc8310261aab0ce9e21e44\u0026order=1","length":0,"order":1,"stream_type":0,"p2p_type":1}]}"#;
        println!("{:?}", serde_json::from_str::<StreamInfoV1>(v1).unwrap());
        println!("{:?}", serde_json::from_str::<StreamInfoV2>(v2).unwrap());
    }
}