use std::{borrow::Cow, str::FromStr};

use api_binding::endpoint_prelude::*;
use derive_builder::Builder;
use derive_getters::Getters;
use serde::Deserialize;

#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct All {
    #[builder(setter(into), default)]
    time: Option<i64>,
    #[builder(setter(into), default)]
    icao24: Option<Icao24>,

    #[builder(default)]
    bounding_box: Option<BoundingBox>,

    #[builder(setter(into), default)]
    extended: Option<i32>,
}

impl All {
    pub fn builder() -> AllBuilder {
        AllBuilder::default()
    }
}

impl Endpoint for All {
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
            .push_opt("icao24", self.icao24.clone())
            .extend_opt(self.bounding_box)
            .push_opt("extended", self.extended);

        params
    }
}

#[derive(Debug, Clone)]
pub struct Icao24 {
    icao: String,
}

impl FromStr for Icao24 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if i64::from_str_radix(s, 16).is_ok() && s.len() == 6 {
            Ok(Self {
                icao: s.to_owned().to_lowercase(),
            })
        } else {
            Err("not hex".into())
        }
    }
}

impl From<&str> for Icao24 {
    fn from(value: &str) -> Self {
        Icao24::from_str(value).unwrap()
    }
}

impl ParamValue<'static> for Icao24 {
    fn as_value(&self) -> Cow<'static, str> {
        self.icao.clone().into()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    lamin: f64,
    lomin: f64,
    lamax: f64,
    lomax: f64,
    counter: u8,
}

impl BoundingBox {
    pub fn new(lamin: f64, lomin: f64, lamax: f64, lomax: f64) -> Self {
        BoundingBox {
            lamin,
            lomin,
            lamax,
            lomax,
            counter: 0,
        }
    }
}

impl Iterator for BoundingBox {
    type Item = (&'static str, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.counter {
            0 => Some(("lamin", self.lamin)),
            1 => Some(("lomin", self.lomin)),
            2 => Some(("lamax", self.lamax)),
            3 => Some(("lomax", self.lomax)),
            _ => None,
        };
        self.counter = if let Some(counter) = self.counter.checked_add(1) {
            counter
        } else {
            u8::MAX
        };

        ret
    }
}

#[derive(Deserialize, Debug, Getters)]
pub struct AllResponse {
    states: Option<Vec<State>>,
    time: u64,
}

#[derive(Deserialize, Debug, Getters)]
pub struct State {
    icao24: String,
    callsign: Option<String>,
    origin_country: String,
    time_position: Option<u64>,
    last_contact: u64,
    longitude: Option<f64>,
    latitude: Option<f64>,
    baro_altitude: Option<f64>,
    on_ground: bool,
    velocity: Option<f64>,
    true_track: Option<f64>,
    vertical_rate: Option<f64>,
    sensors: Option<Vec<i32>>,
    geo_altitude: Option<f64>,
    squawk: Option<String>,
    spi: bool,
    position_source: u8,
    // category: Option<u16>,
}
