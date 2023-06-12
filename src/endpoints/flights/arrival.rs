use api_binding::endpoint_prelude::*;
use derive_builder::Builder;
use derive_getters::Getters;
use serde::Deserialize;

use crate::Flight;

#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct Endpoint {
    airport: String,
    begin: u64,
    end: u64,
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
        "flights/arrival".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push("airport", &self.airport)
            .push("begin", self.begin)
            .push("end", self.end);

        params
    }
}

#[derive(Deserialize, Debug, Getters)]
pub struct Response {
    flights: Vec<Flight>,
}
