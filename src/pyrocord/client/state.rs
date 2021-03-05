use crate::pyrocord::http::http::HTTPClient;
use state::Storage;

pub static HTTP: Storage<HTTPClient> = Storage::new();
