use api_binding::endpoint_prelude::*;
use derive_builder::Builder;
use derive_getters::{Dissolve, Getters};
use serde::Deserialize;

use crate::{Icao24, Track};

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct Endpoint {
    icao24: Icao24,
    #[builder(default)]
    time: Option<u64>,
}

impl Endpoint {
    pub fn builder() -> EndpointBuilder {
        EndpointBuilder::default()
    }
}

impl api_binding::Endpoint for Endpoint {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "tracks".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push("icao24", &self.icao24)
            .push_opt("time", self.time);

        params
    }
}

#[derive(Deserialize, Debug, Getters, Dissolve)]
pub struct Response {
    track: Track,
}
