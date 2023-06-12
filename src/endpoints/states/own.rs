use api_binding::endpoint_prelude::*;
use derive_builder::Builder;
use derive_getters::{Dissolve, Getters};
use serde::Deserialize;

use crate::{Icao24, State};

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct Endpoint {
    #[builder(default)]
    time: Option<i64>,

    #[builder(default)]
    icao24: Option<Icao24>,

    #[builder(default)]
    serials: Option<Vec<u64>>,
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

    fn endpoint(&self) -> std::borrow::Cow<'static, str> {
        "states/own".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("time", self.time)
            .push_opt("icao24", self.icao24.as_ref())
            .extend_opt(
                self.serials
                    .clone()
                    .map(|i| i.into_iter().map(|serial| ("serials", serial))),
            );

        params
    }
}

#[derive(Deserialize, Debug, Getters, Dissolve)]
pub struct Response {
    time: u64,
    states: Vec<State>,
}
