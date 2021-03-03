// use std::ops::Add;
// use serde_json::to_vec;
// use serde::ser::Serialize;
// use reqwest::Method;
use super::routes::Route;
use reqwest::header::{HeaderMap, HeaderValue};

pub struct Request {
    pub headers: Option<HeaderMap<HeaderValue>>,
    pub body: Option<Vec<u8>>,
    pub route: Route,
}

impl Request {} // TODO: short-hand methods e.g. JSON, audit log header
