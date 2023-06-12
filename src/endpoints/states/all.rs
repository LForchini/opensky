use std::borrow::Cow;

use api_binding::endpoint_prelude::*;
use derive_builder::Builder;
use derive_getters::{Dissolve, Getters};
use serde::Deserialize;

use crate::{BoundingBox, Icao24, State};

#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Endpoint {
    #[builder(setter(into), default)]
    time: Option<i64>,
    #[builder(setter(into), default)]
    icao24: Option<Icao24>,

    #[builder(default)]
    bounding_box: Option<BoundingBox>,

    #[builder(setter(into), default)]
    extended: Option<i32>,
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
        "states/all".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("time", self.time)
            .push_opt("icao24", self.icao24.as_ref())
            .extend_type_opt(self.bounding_box)
            .push_opt("extended", self.extended);

        params
    }
}

#[derive(Deserialize, Debug, Getters, Dissolve)]
pub struct Response {
    states: Option<Vec<State>>,
    time: u64,
}
