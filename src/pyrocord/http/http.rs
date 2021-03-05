use super::request::Request;
use http::header::AUTHORIZATION;
use pyo3::prelude::*;
use reqwest::header::HeaderMap;
use reqwest::{Client, Error};
use serde::de::DeserializeOwned;

const BASE_URL: &'static str = "https://discord.com/api/v8";
const USER_AGENT: &'static str =
    "DiscordBot (https://github.com/pyrocord/pyrocord, 0.1.0) rustc/1.52";

#[pyclass]
pub struct HTTPClient {
    client: Client,
}

impl HTTPClient {
    pub fn new(token: String) -> Self {
        let mut headers = HeaderMap::new();

        headers.insert(AUTHORIZATION, format!("Bot {}", token).parse().unwrap());

        let client = Client::builder()
            .user_agent(USER_AGENT)
            .default_headers(headers)
            .build()
            .expect("Could not build Reqwest Client.");

        HTTPClient { client }
    }

    pub async fn request<T: DeserializeOwned>(&self, request: Request) -> Result<T, Error> {
        let (method, url) = request.route.resolve();
        let url = format!("{}{}", BASE_URL, url);

        let response = self
            .client
            .request(method, &url)
            .body(request.body.unwrap_or_default())
            .send()
            .await?;

        // TODO: Ratelimiting

        response.json::<T>().await
    }
}

#[pymethods]
impl HTTPClient {}
