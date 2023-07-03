use bilibili_restapi_model::{*, prelude::*};

#[derive(Clone, Debug, Serialize)]
pub struct GetHostsInfo {
    #[serde(rename(serialize = "id"))]
    pub roomid: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HostsInfo {
    pub host_list: Vec<HostInfo>,
    pub token: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HostInfo {
    pub host: String,
    pub port: u16,
    pub ws_port: u16,
    pub wss_port: u16,
}

impl RestApi for GetHostsInfo {
    const BIZ: BizKind = BizKind::Live;
    const METHOD: RestApiRequestMethod = RestApiRequestMethod::Get;
    const PATH: &'static str = "/xlive/web-room/v1/index/getDanmuInfo";
    const DEFAULT: Option<&'static str> = Some("type=0");
    type Response = HostsInfo;
}
