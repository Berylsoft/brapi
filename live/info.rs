use crate::prelude::*;

#[derive(Clone, Debug, Serialize)]
pub struct GetRoomInfo {
    #[serde(rename = "id")]
    pub sroomid: u32,
}

impl RestApi for GetRoomInfo {
    const BIZ: BizKind = BizKind::Live;
    const METHOD: RestApiRequestMethod = RestApiRequestMethod::BareGet;
    const PATH: &'static str = "/room/v1/Room/get_info";
    const DEFAULT: Option<&'static str> = None;
    type Response = RoomInfo;
}

#[derive(Clone, Debug, Deserialize)]
pub struct RoomInfo {
    pub uid: u64,
    pub room_id: u32,
    pub short_id: u32,
    pub live_status: u8,
    pub parent_area_name: String,
    pub area_name: String,
    pub title: String,
    pub attention: u32,
    pub online: u32,
    pub is_portrait: bool,
    pub description: String,
    pub area_id: u16,
    pub parent_area_id: u8,
    pub background: String,
    pub user_cover: String,
    pub keyframe: String,
    pub tags: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserInfoInfo {
    pub uname: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserInfoLevelMaster {
    pub level: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserInfoLevel {
    pub master_level: UserInfoLevelMaster,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserInfo {
    pub info: UserInfoInfo,
    pub level: UserInfoLevel,
}

#[derive(Clone, Debug, Serialize)]
pub struct GetUserInfo {
    #[serde(rename = "roomid")]
    pub sroomid: u32,
}

impl RestApi for GetUserInfo {
    const BIZ: BizKind = BizKind::Live;
    const METHOD: RestApiRequestMethod = RestApiRequestMethod::BareGet;
    const PATH: &'static str = "/live_user/v1/UserInfo/get_anchor_in_room";
    const DEFAULT: Option<&'static str> = None;
    type Response = UserInfo;
}

#[derive(Clone, Debug, Serialize)]
pub struct GetExtRoomInfo {
    #[serde(rename = "room_id")]
    pub sroomid: u32,
}

impl RestApi for GetExtRoomInfo {
    const BIZ: BizKind = BizKind::Live;
    const METHOD: RestApiRequestMethod = RestApiRequestMethod::BareGet;
    const PATH: &'static str = "/xlive/web-room/v1/index/getH5InfoByRoom";
    const DEFAULT: Option<&'static str> = None;
    type Response = ExtRoomInfo;
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExtRoomInfo {
    pub watched_show: WatchedInfo,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WatchedInfo {
    pub switch: bool,
    pub num: u32,
}
