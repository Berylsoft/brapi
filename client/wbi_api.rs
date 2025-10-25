use brapi_model::{*, prelude::*};

#[derive(Clone, Debug, Serialize)]
pub struct GetBasicInfo;

#[derive(Clone, Debug, Deserialize)]
pub struct BasicInfo {
    pub wbi_img: WbiImg,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WbiImg {
    pub img_url: String,
    pub sub_url: String,
}

impl RestApi for GetBasicInfo {
    const BIZ: BizKind = BizKind::Common { from: None };
    const METHOD: RestApiRequestMethod = RestApiRequestMethod::Get;
    const PATH: &'static str = "/x/web-interface/nav";
    type Response = BasicInfo;
}
