use std::{collections::BTreeMap, sync::Arc};
use serde_urlencoded::to_string as to_urlencoded;
use bytes::Bytes;
use http::{Request, Response, header::{self, HeaderValue, HeaderMap}};
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper_util::{client::legacy::{Client as HyperClient, connect::HttpConnector}, rt::TokioExecutor};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use tokio::sync::RwLock;
use brapi_model::{*, prelude::concat_string};
use crate::{error::*, access::Access, wbi, wbi_api};

pub const WEB_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36";

type FeaturedHyperClient = HyperClient<HttpsConnector<HttpConnector>, Full<Bytes>>;

fn build_hyper_client() -> FeaturedHyperClient {
    let https = HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_only()
        .enable_http2()
        .build();
    HyperClient::builder(TokioExecutor::new()).build(https)
}

pub struct Client {
    hyper: FeaturedHyperClient,
    access: Option<Access>,
    proxy: BTreeMap<BizKind, String>,
    wbi_key: RwLock<Option<[u8; 32]>>,
}

pub type ClientRef = Arc<Client>;

impl Client {
    fn _new(access: Option<Access>, proxy: Option<BTreeMap<BizKind, String>>) -> ClientRef {
        let client = Client {
            hyper: build_hyper_client(),
            access,
            proxy: proxy.unwrap_or_default(),
            wbi_key: RwLock::new(None),
        };
        Arc::new(client)
    }

    pub fn with_access(access: String, proxy: Option<BTreeMap<BizKind, String>>) -> Option<ClientRef> {
        let access = Access::from_raw(access)?;
        Some(Client::_new(Some(access), proxy))
    }

    pub fn without_access(proxy: Option<BTreeMap<BizKind, String>>) -> ClientRef {
        Client::_new(None, proxy)
    }

    pub fn bare() -> ClientRef {
        Client::_new(None, None)
    }

    pub fn uid(&self) -> Option<u64> {
        self.access.as_ref().map(|access| access.uid)
    }

    pub fn devid3(&self) -> Option<String> {
        self.access.as_ref().map(|access| access.devid3.clone())
    }

    fn set_headers(&self, biz: BizKind, headers: &mut HeaderMap) {
        headers.insert(header::REFERER, HeaderValue::from_static(biz.referer()));
        headers.insert(header::ORIGIN, HeaderValue::from_static(biz.referer()));
        headers.insert(header::USER_AGENT, HeaderValue::from_static(WEB_USER_AGENT));
        if let Some(access) = &self.access {
            let mut cookie = HeaderValue::from_str(&access.raw).unwrap();
            cookie.set_sensitive(true);
            headers.insert(header::COOKIE, cookie);
        }
    }

    // pub fn host<'a>(&self, biz: BizKind) -> &'a str {
    // }

    pub fn csrf(&self) -> RestApiResult<&str> {
        match &self.access {
            Some(access) => Ok(access.csrf.as_str()),
            None => Err(RestApiError::PostWithoutAccess),
        }
    }

    pub fn clone_raw(&self) -> FeaturedHyperClient {
        self.hyper.clone()
    }

    pub async fn raw_get(&self, url: http::Uri) -> HyperClientResult<Response<Incoming>> {
        self.hyper.get(url).await
    }

    pub async fn raw_get_bytes(&self, url: http::Uri) -> RestApiResult<Response<Bytes>> {
        let (parts, body) = self.hyper.get(url).await?.into_parts();
        let body = body.collect().await?.to_bytes();
        Ok(Response::from_parts(parts, body))
    }

    pub async fn update_wbi_key(&self) -> RestApiResult<()> {
        let basic_info = self.call(&wbi_api::GetBasicInfo).await?;
        let new_key = wbi::get_key(basic_info)?;
        let mut key_ref = self.wbi_key.write().await;
        key_ref.replace(new_key);
        Ok(())
    }

    pub async fn call<Req: RestApi>(&self, req: &Req) -> RestApiResult<Req::Response> {
        let host = match self.proxy.get(&Req::BIZ) {
            None => Req::BIZ.host(),
            Some(proxy) => proxy.as_str(),
        };
        let mut url = concat_string!(host, Req::PATH);

        let req = match Req::METHOD {
            RestApiRequestMethod::BareGet | RestApiRequestMethod::Get => {
                let mut orig_params = to_urlencoded(req)?;
                if let Some(default) = Req::DEFAULT {
                    orig_params.push_str("&");
                    orig_params.push_str(default);
                }
                let params = if Req::WBI {
                    let (dir, ts, _) = foundations::now::now_raw();
                    assert!(dir);
                    let key_ref = &self.wbi_key.read().await.ok_or(RestApiError::NoWbiKey)?;
                    wbi::sign(orig_params, key_ref, ts)?
                } else {
                    orig_params
                };
                url.push_str("?");
                url.push_str(&params);

                let mut _req = Request::get(url);

                if let RestApiRequestMethod::Get = Req::METHOD {
                    let headers = _req.headers_mut().unwrap();
                    self.set_headers(Req::BIZ, headers);
                }

                _req.body(Full::new(Bytes::new()))
            },
            RestApiRequestMethod::PostForm => {
                let mut _req = Request::post(url);

                let csrf = self.csrf()?;
                let urlencoded = to_urlencoded(req)?;
                let mut body = concat_string!("csrf=", csrf, "&csrf_token={}", csrf);
                if let Some(default) = Req::DEFAULT {
                    body.push('&');
                    body.push_str(default);
                }
                if urlencoded.is_empty() {
                    body.push('&');
                    body.push_str(&urlencoded);
                }

                let headers = _req.headers_mut().unwrap();
                headers.insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("application/x-www-form-urlencoded"),
                );
                self.set_headers(Req::BIZ, headers);

                _req.body(Full::new(body.into()))
            },
            RestApiRequestMethod::PostJson => {
                assert_eq!(Req::DEFAULT, None);
                unimplemented!()
            }
        }.unwrap();

        let resp = self.hyper.request(req).await?;
        let status = resp.status().as_u16();
        let body = resp.collect().await?.to_bytes();
        let text = std::str::from_utf8(body.as_ref())?;

        if status == 200 {
            let RestApiResponse { code, message, data }: RestApiResponse<Req::Response> = serde_json::from_str(text)?;
            if code == 0 {
                Ok(data)
            } else {
                Err(RestApiError::Failure {
                    code: RestApiFailureCode::FromApi { code, message },
                    payload: text.to_owned(),
                    rate_limited: code == -412,
                })
            }
        } else {
            Err(RestApiError::Failure {
                code: RestApiFailureCode::FromHttp(status),
                payload: text.to_owned(),
                rate_limited: status == 412,
            })
        }
    }
}
