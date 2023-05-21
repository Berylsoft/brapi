use bilibili_restapi_model::{*, prelude::*};

#[derive(Clone, Debug, Serialize)]
pub struct GetTagInfo {
    #[serde(rename(serialize = "tag_id"))]
    pub tagid: u32,
}

impl RestApi for GetTagInfo {
    const BIZ: BizKind = BizKind::Common { from_page: CommonFromPageKind::Dynamic };
    const METHOD: RestApiRequestMethod = RestApiRequestMethod::BareGet;
    const PATH: &'static str = "/x/tag/info";
    const DEFAULT: Option<&'static str> = None;
    type Response = TagInfo;
}

#[derive(Clone, Debug, Deserialize)]
pub struct TagInfo {
    pub tag_id: u32,
    pub tag_name: String,
    #[cfg(feature = "unclarified_fields")]
    pub cover: String,
    #[cfg(feature = "unclarified_fields")]
    pub head_cover: String,
    #[cfg(feature = "unclarified_fields")]
    pub content: String,
    #[cfg(feature = "unclarified_fields")]
    pub short_content: String,
    #[cfg(feature = "unclarified_fields")]
    pub r#type: i32,
    #[cfg(feature = "unclarified_fields")]
    pub state: i32,
    #[serde(rename(serialize = "ctime"))]
    /// unix timestamp (s)
    pub create_time: u32,
    pub count: TagInfoCount,
    #[cfg(feature = "unclarified_fields")]
    pub is_atten: i32, // 0 or 1
    #[cfg(feature = "unclarified_fields")]
    pub likes: i32,
    #[cfg(feature = "unclarified_fields")]
    pub hates: i32,
    #[cfg(feature = "unclarified_fields")]
    pub attribute: i32,
    #[cfg(feature = "unclarified_fields")]
    pub liked: i32,
    #[cfg(feature = "unclarified_fields")]
    pub hated: i32,
    #[cfg(feature = "unclarified_fields")]
    pub extra_attr: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TagInfoCount {
    #[serde(rename(serialize = "view"))]
    pub viewed: u32,
    #[serde(rename(serialize = "use"))]
    pub used: u32,
    #[serde(rename(serialize = "atten"))]
    pub follows: u32,
}

#[derive(Clone, Debug, Serialize)]
pub struct GetExtTagInfo {
    #[serde(rename(serialize = "topic_id"))]
    pub tagid: u32,
}

impl RestApi for GetExtTagInfo {
    const BIZ: BizKind = BizKind::LegacyDynamic;
    const METHOD: RestApiRequestMethod = RestApiRequestMethod::BareGet;
    const PATH: &'static str = "/topic_svr/v1/topic_svr/get_active_users";
    const DEFAULT: Option<&'static str> = None;
    type Response = ExtTagInfo;
}

#[derive(Clone, Debug, Deserialize)]
pub struct ExtTagInfo {
    #[serde(rename(serialize = "topic_id"))]
    pub tagid: u32,
    pub view_count: u32,
    pub discuss_count: u32,
    pub active_users: Vec<TagActiveUser>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TagActiveUser {
    // todo
}
