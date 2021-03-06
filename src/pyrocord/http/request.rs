use super::routes::Route;
use super::parameters::Parameters;
use reqwest::header::{HeaderMap, HeaderValue};

pub struct Request {
    pub headers: Option<HeaderMap<HeaderValue>>,
    pub body: Option<Vec<u8>>,
    pub route: Route,
    // TODO: Parameters enum -> [(key, value)]
    pub parameters: Option<Parameters>,
}

impl Request {} // TODO: short-hand methods e.g. JSON, audit log header
