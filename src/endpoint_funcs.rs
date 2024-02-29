use crate::{Message, WebhookList};
use std::convert::Infallible;
use warp::{reject, reply, sse::Event, Rejection, Reply};

pub fn sse_counter(counter: String) -> Result<Event, Infallible> {
    Ok(warp::sse::Event::default().data(counter))
}

pub async fn send(message: Message, ids: WebhookList) -> Result<impl Reply, Rejection> {
    let sender = match ids.get_id(message.destnation.to_string()) {
        Some(dat) => dat.0,
        None => return Err(reject()),
    };

    let _ = sender.send(serde_json::to_string(&message).unwrap());

    ids.send(message);

    Ok(reply::html("ok"))
}

pub async fn issue_id(ids: WebhookList, name: String) -> Result<impl Reply, Rejection> {
    let id = ids.issue_id(name);

    Ok(reply::json(&id))
}

pub async fn new_connecion(ids: WebhookList, username: String) -> Result<impl Reply, Rejection> {
    ids.anounce_new_user(username);

    Ok(warp::reply::json(&String::new()))
}

pub async fn get_history(ids: WebhookList, channel_id: String) -> Result<impl Reply, Rejection> {
    let history = match ids.get_channel_history(channel_id) {
        Some(dat) => dat,
        None => return Err(reject()),
    };

    Ok(warp::reply::json(&serde_json::to_string(&history).unwrap()))
}
