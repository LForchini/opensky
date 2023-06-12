use api_binding::client_prelude::*;
use thiserror::Error;

#[derive(Debug)]
pub struct OpenSky {
    client: reqwest::blocking::Client,
    rest_url: Url,
}

impl OpenSky {
    pub fn new() -> Result<Self, OpenSkyError> {
        let rest_url = Url::parse("https://opensky-network.org/api/")?;
        let client = reqwest::blocking::Client::new();

        Ok(OpenSky { rest_url, client })
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
            let request = http_request.try_into()?;
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
}

impl OpenSkyError {
    fn url_parse(source: ParseError) -> Self {
        OpenSkyError::Url { source }
    }

    fn client(source: reqwest::Error) -> Self {
        OpenSkyError::Client { source }
    }

    fn http(source: HttpError) -> Self {
        OpenSkyError::Http { source }
    }
}
