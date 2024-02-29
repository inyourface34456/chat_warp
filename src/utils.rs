use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};
use warp::{body, Filter, Rejection};

pub fn json_string() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    body::content_length_limit(1024 * 16).and(body::json())
}

pub fn json_message() -> impl Filter<Extract = (Message,), Error = Rejection> + Clone {
    body::content_length_limit(1024 * 16).and(body::json())
}

pub type Outer<T> = Arc<RwLock<T>>;
pub const LENGTH: usize = 100;

pub fn to_outer<T>(data: T) -> Outer<T> {
    Arc::new(RwLock::new(data))
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub username: String,
    pub message: String,
    pub destnation: String,
}

#[derive(Clone, Serialize)]
pub struct ShortMessage {
    pub username: String,
    pub message: String,
}

impl From<Message> for ShortMessage {
    fn from(s: Message) -> ShortMessage {
        ShortMessage {
            username: s.username,
            message: s.message,
        }
    }
}

pub struct MessageHistory<const LENGTH: usize> {
    history: VecDeque<ShortMessage>,
    length: usize,
}

impl<const LENGTH: usize> MessageHistory<LENGTH> {
    pub fn push(&mut self, data: ShortMessage) {
        if self.history.len() > self.length {
            for _ in 0..self.history.len() - self.length {
                self.history.pop_front();
            }
        }

        self.history.push_back(data);
    }

    pub fn history(&self) -> VecDeque<ShortMessage> {
        self.history.clone()
    }
}
