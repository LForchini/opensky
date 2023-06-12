use api_binding::endpoint_prelude::*;
use derive_builder::Builder;
use derive_getters::{Dissolve, Getters};
use serde::Deserialize;

use crate::{Flight, Icao24};

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct Endpoint {
    begin: u64,
    end: u64,
    icao24: Icao24,
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
        "flights/aircraft".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push("begin", self.begin)
            .push("end", self.end)
            .push("icao24", &self.icao24);

        params
    }
}

#[derive(Deserialize, Debug, Getters, Dissolve)]
pub struct Response {
    flights: Vec<Flight>,
}
