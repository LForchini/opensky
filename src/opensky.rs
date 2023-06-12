use api_binding::client_prelude::*;
use base64::Engine;
use reqwest::header::{HeaderValue, InvalidHeaderValue};
use thiserror::Error;

#[derive(Debug)]
pub struct OpenSky {
    async_client: reqwest::Client,
    client: reqwest::blocking::Client,
    auth: Option<String>,
    rest_url: Url,
}

impl OpenSky {
    pub fn new() -> Result<Self, OpenSkyError> {
        let rest_url = Url::parse("https://opensky-network.org/api/")?;
        let client = reqwest::blocking::Client::new();
        let async_client = reqwest::Client::new();

        Ok(OpenSky {
            rest_url,
            auth: None,
            client,
            async_client,
        })
    }

    pub fn set_auth(&mut self, auth: impl ToString) {
        let data = base64::engine::general_purpose::STANDARD_NO_PAD.encode(auth.to_string());

        self.auth = Some(format!("Basic {}", data));
    }
}

impl RestClient for OpenSky {
    type Error = OpenSkyError;

    fn rest_endpoint(&self, endpoint: &str) -> Result<Url, ApiError<Self::Error>> {
        self.rest_url
            .join(endpoint)
            .map_err(OpenSkyError::url_parse)
            .map_err(ApiError::client)
    }
}

impl Client for OpenSky {
    fn rest(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, ApiError<Self::Error>> {
        let call = || -> Result<_, OpenSkyError> {
            let http_request = request.body(body)?;
            let mut request: reqwest::blocking::Request = http_request.try_into()?;
            if let Some(auth) = &self.auth {
                let headers = request.headers_mut();
                headers.insert("Authorization", HeaderValue::from_str(&auth)?);
            }
            let rsp = self.client.execute(request)?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
            }

            Ok(http_rsp.body(rsp.bytes()?)?)
        };

        call().map_err(ApiError::client)
    }
}

#[async_trait]
impl AsyncClient for OpenSky {
    async fn rest_async(
        &self,
        request: RequestBuilder,
        body: Vec<u8>,
    ) -> Result<HttpResponse<Bytes>, ApiError<Self::Error>> {
        let call = async {
            let http_request = request.body(body)?;
            let mut request: reqwest::Request = http_request.try_into()?;
            if let Some(auth) = &self.auth {
                let headers = request.headers_mut();
                headers.insert("Authorization", HeaderValue::from_str(&auth)?);
            }
            let rsp = self.async_client.execute(request).await?;

            let mut http_rsp = HttpResponse::builder()
                .status(rsp.status())
                .version(rsp.version());
            let headers = http_rsp.headers_mut().unwrap();
            for (key, value) in rsp.headers() {
                headers.insert(key, value.clone());
            }

            Ok(http_rsp.body(rsp.bytes().await?)?)
        };

        call.await.map_err(ApiError::client)
    }
}

#[derive(Debug, Error)]
pub enum OpenSkyError {
    #[error("failed to parse url: {}", source)]
    Url {
        #[from]
        source: ParseError,
    },

    #[error("failed to create http client: {}", source)]
    Client {
        #[from]
        source: reqwest::Error,
    },

    #[error("failed to create request: {}", source)]
    Http {
        #[from]
        source: HttpError,
    },

    #[error("failed to create header: {}", source)]
    Header {
        #[from]
        source: InvalidHeaderValue,
    },
}

impl OpenSkyError {
    fn url_parse(source: ParseError) -> Self {
        OpenSkyError::Url { source }
    }
}
