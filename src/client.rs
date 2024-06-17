use std::collections::HashMap;

use async_std::task;
use bytes::Bytes;
use maybe_async::maybe_async;

use Format::Xml;
use maybe_http_client::{HttpClient, HttpClientError};

use crate::common::{Format, Function};
use crate::common::error::ModelError;
use crate::common::models::{ActiveSearchResult, Caps, NewznabError, NewznabRawError, SearchResult};
use crate::Error;

pub struct ClientBuilder {
    url: Option<String>,
    endpoint: String,
    api_token: Option<String>,
    caps: Caps,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            url: None,
            endpoint: "/api".to_string(),
            api_token: None,
            caps: Default::default(),
        }
    }
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn url(mut self, value: impl AsRef<str>) -> Self {
        self.url = Some(value.as_ref().to_string());
        self
    }
    pub fn endpoint(mut self, value: impl AsRef<str>) -> Self {
        self.endpoint = value.as_ref().to_string();
        self
    }
    pub fn api_token(mut self, value: impl AsRef<str>) -> Self {
        self.api_token = Some(value.as_ref().to_string());
        self
    }

    pub fn to_client(self) -> Client {
        self.into()
    }
}

#[cfg_attr(target_arch = "wasm32", maybe_async(? Send))]
#[cfg_attr(not(target_arch = "wasm32"), maybe_async(AFIT))]
impl Into<Client> for ClientBuilder {
    fn into(self) -> Client {
        let mut c = Client {
            url: self.url.expect("An 'url' needs to be specified"),
            endpoint: self.endpoint,
            api_token: self.api_token,
            http: HttpClient::default(),
            caps: Default::default(),
        };

        #[cfg(feature = "async")]
        task::block_on(async {
            if let Ok(caps) = c.get_caps().await {
                c.caps = caps;
            }
        });

        #[cfg(feature = "sync")]
        if let Ok(caps) = c.get_caps() {
            c.caps = caps;
        }

        c
    }
}


#[derive(Debug, Clone)]
pub struct Client {
    pub(crate) url: String,
    pub(crate) endpoint: String,
    pub(crate) api_token: Option<String>,
    pub(crate) http: HttpClient,
    pub(crate) caps: Caps,
}


// #[maybe_async::maybe_async(AFIT)]
impl Client {
    pub fn get_api_url(&self) -> String {
        let mut base = self.url.clone();
        if !base.ends_with('/') && !self.endpoint.starts_with("/") {
            base.push('/');
        } else if base.ends_with('/') && self.endpoint.starts_with("/") {
            let mut chars = base.chars();
            // chars.next();
            chars.next_back();
            base = chars.as_str().to_string()
        }
        // (base + self.endpoint.as_ref()) + "?o=json"
        base + self.endpoint.as_ref()
    }

    pub fn get_http(&self) -> &HttpClient {
        &self.http
    }

    pub fn get_api_key(&self) -> Option<&String> {
        self.api_token.as_ref()
    }

    pub fn get_default_payload(&self) -> HashMap<String, String> {
        let mut payload = HashMap::new();
        // payload.insert("o", "json");
        if self.api_token.is_some() {
            payload.insert("apikey".to_string(), self.api_token.as_ref().unwrap().to_string().clone());
        }
        payload.into()
    }

    /// Returns the absolute URL for an endpoint in the API.
    pub fn endpoint_url(&self, endpoint: impl AsRef<str>) -> String {
        let mut base = self.get_api_url();
        if !base.ends_with('/') && !endpoint.as_ref().starts_with("/") {
            base.push('/');
        }
        base.to_owned() + endpoint.as_ref()
    }


    #[maybe_async::maybe_async(AFIT)]
    pub async fn function(&self, f: Function, o: Format) -> Result<String, Error> {
        let mut payload = self.get_default_payload();

        match o {
            Xml => { payload.insert("o".to_string(), "xml".to_string()); }
            Format::Json => { payload.insert("o".to_string(), "json".to_string()); }
        }

        payload.insert("t".to_string(), f.to_string());

        match f {
            Function::Caps => {}
            Function::Register { .. } => {}
            Function::Search(p) => {
                payload.insert("q".to_string(), p.q);

                if let Some(value) = p.limit {
                    payload.insert("limit".to_string(), value.to_string());
                } else {
                    payload.insert("limit".to_string(), self.caps.limits.max().to_string());
                }
                if let Some(value) = p.offset {
                    payload.insert("offset".to_string(), value.to_string());
                }

                if let Some(params) = p.params {
                    for (k, v) in params {
                        payload.insert(k, v);
                    }

                }
            }
        }

        let resp = self.http.get(
            self.get_api_url().as_str(),
            None,
            &payload.iter().map(
                |(k, v)| { (k.clone(), v.clone()) }
            ).collect(),
        ).await;

        match resp {
            Ok(data) => {
                match serde_json::from_str::<NewznabRawError>(data.as_str()) {
                    Ok(raw) => {
                        let e: NewznabError = raw.into();
                        // Err(err)
                        log::error!("Error: {}", e);
                        Err(Error::from(e))
                    }
                    Err(e) => {
                        // means we got no error
                        // println!("json_error: {:?}", e);
                        log::debug!("Request successfull");
                        Ok(data)
                    }
                }
            }
            // Err(ReqwestError::Io(e)) => {
            //     eprintln!("failed to decode response: {}", &e);
            //     Err(Error::from(e))
            // },
            Err(HttpClientError::StatusCode(response)) => {
                let code = response.status();
                let msg = "";
                Err(
                    Error::HttpStatusCode(
                        code.as_u16(),
                        String::from_utf8_lossy(&*response.bytes().await.unwrap()).to_string(),
                    )
                )
            }
            _ => {
                Err(Error::Http(Box::new(resp.unwrap_err())))
            }
        }
    }

    #[maybe_async::maybe_async]
    pub async fn get_caps(&self) -> Result<Caps, Error> {
        Ok(
            Caps::try_from(
                self.function(
                    Function::Caps,
                    Xml,
                ).await?
            ).map_err(|e| { ModelError::from(e) })?
        )
    }
    // async fn register(&self) -> Result<String, Self::Error>;
    //
    #[maybe_async::maybe_async]
    pub async fn search(&self, f: Function) -> Result<ActiveSearchResult, Error> {
        let res = self.function(f.clone(), Xml).await;
        if let Ok(xml_str) = res {
            let sres = SearchResult::try_from(xml_str.as_str());
            if let Ok(sr) = sres {
                Ok(ActiveSearchResult {
                    // client: &self,
                    search_parameters: f.unwrap_search(),
                    search_offset: sr.offset,
                    // fetch_size: sr.items.len(),
                    items: sr.items,
                })
                // Ok(sr)
            } else {
                Err(Error::from(sres.unwrap_err()))
            }
        } else {
            Err(res.unwrap_err())
        }
    }
}