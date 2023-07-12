#![allow(non_camel_case_types)] // for `FormatKind` only but invalid after proc macro of `serde-enum-str`

use brapi_model::{*, prelude::*};

#[derive(Clone, Debug, Serialize)]
pub struct GetStreamInfoV1 {
    #[serde(rename(serialize = "cid"))]
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
    #[serde(rename(deserialize = "durl"))]
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
    #[serde(rename(serialize = "room_id"))]
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
    #[serde(rename(deserialize = "room_id"))]
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
    #[serde(rename(deserialize = "playurl_info"))]
    pub urls: Option<StreamUrlsV2>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StreamUrlsV2 {
    #[cfg(feature = "unclarified_fields")]
    /// `"{\"cdn_rate\":10000,\"report_interval_sec\":150}"`
    pub conf_json: JsonValue,
    #[serde(rename(deserialize = "playurl"))]
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
    #[serde(rename(deserialize = "roomid"))]
    pub cid: u32,
    #[cfg(feature = "useless_fields")]
    pub g_qn_desc: Vec<QnDescV2>,
    #[serde(rename(deserialize = "stream"))]
    pub protocols: Vec<StreamUrlsV2Protocol>,
    #[cfg(feature = "unclarified_fields")]
    /// `{"p2p":true,"p2p_type":1,"m_p2p":false,"m_servers":null}`
    pub p2p_data: JsonValue,
    #[cfg(feature = "unclarified_fields")]
    /// `null`
    pub dolby_qn: JsonValue,
}

#[derive(Clone, Debug, Deserialize_enum_str, PartialEq)]
pub enum ProtocolKind {
    #[serde(rename(deserialize = "http_stream"))]
    HttpStream,
    #[serde(rename(deserialize = "http_hls"))]
    HttpHLS,
    #[serde(other)]
    Unknown(String),
}

#[derive(Clone, Debug, Deserialize)]
pub struct StreamUrlsV2Protocol {
    #[serde(rename(deserialize = "protocol_name"))]
    pub protocol_kind: ProtocolKind,
    #[serde(rename(deserialize = "format"))]
    pub formats: Vec<StreamUrlsV2Format>,
}

#[derive(Clone, Debug, Deserialize_enum_str, PartialEq)]
pub enum FormatKind {
    #[serde(rename(deserialize = "flv"))]
    HTTP_FLV,
    #[serde(rename(deserialize = "ts"))]
    HLS_TS,
    #[serde(rename(deserialize = "fmp4"))]
    HLS_FMP4,
    #[serde(other)]
    Unknown(String),
}

#[derive(Clone, Debug, Deserialize)]
pub struct StreamUrlsV2Format {
    #[serde(rename(deserialize = "format_name"))]
    pub format_kind: FormatKind,
    #[serde(rename(deserialize = "codec"))]
    pub codecs: Vec<StreamUrlsV2Codec>,
}

#[derive(Clone, Debug, Deserialize_enum_str, PartialEq)]
#[allow(non_camel_case_types)]
pub enum CodecKind {
    #[serde(rename(deserialize = "avc"))]
    AVC,
    #[serde(rename(deserialize = "hevc"))]
    HEVC,
    #[serde(other)]
    Unknown(String),
}

#[derive(Clone, Debug, Deserialize)]
pub struct StreamUrlsV2Codec {
    #[serde(rename(deserialize = "codec_name"))]
    pub codec_kind: CodecKind,
    #[serde(flatten)]
    pub urls: StreamUrlV2,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StreamUrlV2 {
    pub current_qn: i32,
    pub accept_qn: Vec<i32>,
    pub base_url: String,
    #[serde(rename(deserialize = "url_info"))]
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
    #[serde(rename(deserialize = "extra"))]
    pub token: String,
    #[cfg(feature = "unclarified_fields")]
    /// `3600`
    pub stream_ttl: JsonValue,
}
