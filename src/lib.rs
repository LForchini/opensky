mod endpoints;
mod opensky;
mod structs;

pub use endpoints::flights;
pub use endpoints::states;
pub use endpoints::track;

pub use opensky::OpenSky;

pub use structs::*;

pub use api_binding::{AsyncClient, AsyncQuery, Client, Endpoint, Query};
