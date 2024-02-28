use std::sync::{Arc, RwLock};
use warp::{body, Filter, Rejection};
use serde::{Deserialize, Serialize};

pub fn json_string() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    body::content_length_limit(1024 * 16).and(body::json())
}

pub fn json_message() -> impl Filter<Extract = (Message,), Error = Rejection> + Clone {
    body::content_length_limit(1024 * 16).and(body::json())
}

pub type Outer<T> = Arc<RwLock<T>>;

pub fn to_outer<T>(data: T) -> Outer<T> {
    Arc::new(RwLock::new(data))
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub username: String,
    pub message: String,
    pub destnation: String
}