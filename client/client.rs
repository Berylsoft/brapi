use std::{collections::BTreeMap, sync::Arc};
use serde_urlencoded::to_string as to_urlencoded;
use hyper::{Request, Response, Body, header::{self, HeaderValue, HeaderMap}};
use brapi_model::{*, prelude::concat_string};
use crate::{error::*, access::Access};

pub const WEB_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/141.0.0.0 Safari/537.36";

pub type HyperClient = hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>;

fn build_hyper_client() -> HyperClient {
    let conn = hyper_rustls::HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_only()
        .enable_http2()
        .build();
    hyper::Client::builder().build(conn)
}

pub struct Client {
    client: HyperClient,
    access: Option<Access>,
    proxy: BTreeMap<BizKind, String>,
}

pub type ClientRef = Arc<Client>;

impl Client {
    fn _new(access: Option<Access>, proxy: Option<BTreeMap<BizKind, String>>) -> ClientRef {
        let client = Client {
            client: build_hyper_client(),
            access,
            proxy: proxy.unwrap_or_default(),
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

    pub fn clone_raw(&self) -> HyperClient {
        self.client.clone()
    }

    pub async fn raw_get(&self, url: hyper::Uri) -> HttpResult<Response<Body>> {
        self.client.get(url).await
    }

    pub async fn call<Req: RestApi>(&self, req: &Req) -> RestApiResult<Req::Response> {
        let host = match self.proxy.get(&Req::BIZ) {
            None => Req::BIZ.host(),
            Some(proxy) => proxy.as_str(),
        };
        let mut url = concat_string!(host, Req::PATH);

        let req = match Req::METHOD {
            RestApiRequestMethod::BareGet | RestApiRequestMethod::Get => {
                let urlencoded = to_urlencoded(req)?;
                if Req::DEFAULT.is_some() || !urlencoded.is_empty() {
                    url.push('?');
                    url.push_str(&urlencoded);
                    if let Some(default) = Req::DEFAULT {
                        url.push('&');
                        url.push_str(default);
                    }
                }

                let mut _req = Request::get(url);

                if let RestApiRequestMethod::Get = Req::METHOD {
                    let headers = _req.headers_mut().unwrap();
                    self.set_headers(Req::BIZ, headers);
                }

                _req.body(Body::empty())
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

                _req.body(Body::from(body))
            },
            RestApiRequestMethod::PostJson => {
                assert_eq!(Req::DEFAULT, None);
                unimplemented!()
            }
        }.unwrap();

        let resp = self.client.request(req).await?;
        let status = resp.status().as_u16();
        let bytes = hyper::body::to_bytes(resp.into_body()).await?;
        let text = std::str::from_utf8(bytes.as_ref())?;

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
