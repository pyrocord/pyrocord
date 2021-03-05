use crate::pyrocord::http::client::HTTPClient;
use state::Storage;

pub static HTTP: Storage<HTTPClient> = Storage::new();
