use std::{borrow::Cow, str::FromStr};

use api_binding::{ParamType, ParamValue};
use derive_getters::Getters;
use serde::Deserialize;

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

impl ParamValue<'static> for &Icao24 {
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
}

impl BoundingBox {
    pub fn new(lamin: f64, lomin: f64, lamax: f64, lomax: f64) -> Self {
        BoundingBox {
            lamin,
            lomin,
            lamax,
            lomax,
        }
    }
}

impl ParamType<'static> for BoundingBox {
    fn as_pairs(&self) -> Vec<(Cow<'static, str>, Cow<'static, str>)> {
        vec![
            ("lamin".into(), self.lamin.to_string().into()),
            ("lomin".into(), self.lomin.to_string().into()),
            ("lamax".into(), self.lamax.to_string().into()),
            ("lomax".into(), self.lomax.to_string().into()),
        ]
    }
}

#[derive(Deserialize, Debug, Getters, Clone)]
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

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.icao24 == other.icao24
    }
}

#[derive(Deserialize, Debug, Getters)]
pub struct Flight {
    icao24: String,
    first_seen: u64,
    est_departure_airport: Option<String>,
    last_seen: u64,
    est_arrival_airport: Option<String>,
    callsign: Option<String>,
    est_departure_airport_horiz_distance: Option<u64>,
    est_departure_airport_vert_distance: Option<u64>,
    est_arrival_airport_horiz_distance: Option<u64>,
    est_arrival_airport_vert_distance: Option<u64>,
    departure_airport_candidates_count: u64,
    arrival_airport_candidates_count: u64,
}

#[derive(Deserialize, Debug, Getters)]
pub struct Track {
    icao24: String,
    start_time: u64,
    end_time: u64,
    callsign: String,
    path: Vec<Waypoint>,
}

#[derive(Deserialize, Debug, Getters)]
pub struct Waypoint {
    time: u64,
    latitude: Option<f64>,
    longitude: Option<f64>,
    baro_altitude: Option<f64>,
    true_track: Option<f64>,
    on_ground: bool,
}
