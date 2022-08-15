use std::collections::BTreeMap;
use serde_urlencoded::to_string as to_urlencoded;
use hyper::{Request, Response, Body, header::{self, HeaderValue, HeaderMap}};
use bilibili_restapi_model::{*, prelude::concat_string};
use crate::{error::*, access::Access};

pub type HyperClient = hyper::Client<hyper_rustls::HttpsConnector<hyper::client::connect::HttpConnector>>;

fn build_hyper_client() -> HyperClient {
    let conn = hyper_rustls::HttpsConnectorBuilder::new()
        .with_webpki_roots()
        .https_only()
        .enable_http1()
        .build();
    hyper::Client::builder().build(conn)
}

#[derive(Clone)]
pub struct Client {
    client: HyperClient,
    access: Option<Access>,
    proxy: BTreeMap<BizKind, String>,
}

impl Client {
    pub fn new(access: Option<Access>, proxy: Option<BTreeMap<BizKind, String>>) -> Client {
        Client {
            client: build_hyper_client(),
            access,
            proxy: proxy.unwrap_or_default(),
        }
    }

    pub fn new_bare() -> Client {
        Client::new(None, None)
    }

    pub fn set_headers(&self, biz: BizKind, headers: &mut HeaderMap) {
        headers.insert(header::REFERER, HeaderValue::from_static(referer(biz)));
        headers.insert(header::ORIGIN, HeaderValue::from_static(referer(biz)));
        headers.insert(header::USER_AGENT, HeaderValue::from_static(WEB_USER_AGENT));
        if let Some(access) = &self.access {
            let mut cookie = HeaderValue::from_str(access.as_cookie().as_str()).unwrap();
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

    pub async fn raw_get_url(&self, url: hyper::Uri) -> HttpResult<Response<Body>> {
        self.client.get(url).await
    }

    pub async fn call<Req: RestApi>(&self, req: &Req) -> RestApiResult<Req::Response> {
        let host = match self.proxy.get(&Req::BIZ) {
            None => api_host(Req::BIZ),
            Some(proxy) => proxy.as_str(),
        };
        let url = concat_string!(host, Req::PATH);

        let req = match Req::METHOD {
            RestApiRequestMethod::BareGet | RestApiRequestMethod::Get => {
                let urlencoded = to_urlencoded(req)?;
                let url = if matches!(Req::DEFAULT, None) && urlencoded.len() == 0 {
                    url
                } else {
                    let mut url = url;
                    url.push('?');
                    url.push_str(&urlencoded);
                    if let Some(default) = Req::DEFAULT {
                        url.push('&');
                        url.push_str(default);
                    }
                    url
                };

                let mut _req = Request::get(url);

                if matches!(Req::METHOD, RestApiRequestMethod::Get) {
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
                if urlencoded.len() != 0 {
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
