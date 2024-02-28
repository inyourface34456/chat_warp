mod endpoint_funcs;
mod utils;
mod webhook_list;

use endpoint_funcs::*;
use futures_util::StreamExt;
use tokio::sync::broadcast::{self, Receiver};
use tokio_stream::wrappers::BroadcastStream;
use utils::*;
use warp::Filter;
use webhook_list::*;

#[tokio::main]
async fn main() {
    let ids = WebhookList::load("ids.txt".into());
    let ids_filter = warp::any().map(move || ids.clone());

    let listen = warp::path!("webhook" / String / "listen")
        .and(warp::get())
        .and(warp::post())
        .and(ids_filter.clone())
        .map(move |id, id_list: WebhookList| {
            let id_exist;
            let rx2: Option<Receiver<String>> = match id_list.get_id(id) {
                Some(dat) => Some(dat.1),
                None => None,
            };

            let stream = match rx2 {
                Some(rx2) => {
                    id_exist = true;
                    BroadcastStream::new(rx2)
                }
                None => {
                    id_exist = false;
                    BroadcastStream::new(broadcast::channel(16).1)
                }
            };

            let event_stream = stream.map(move |x| {
                if id_exist {
                    match x {
                        Ok(x) => sse_counter(x),
                        Err(err) => sse_counter(err.to_string()),
                    }
                } else {
                    sse_counter("not found".into())
                }
            });

            warp::sse::reply(event_stream)
        });

    let new_channel_listener = warp::path("channel")
        .and(warp::post())
        .and(ids_filter.clone())
        .map(move |id_list: WebhookList| {
            let rx = id_list.get_new_channel_listener();
            let stream = BroadcastStream::new(rx);

            let event_stream = stream.map(move |data| match data {
                Ok(channel) => sse_counter(channel),
                Err(err) => sse_counter(err.to_string()),
            });

            warp::sse::reply(event_stream)
        });

    let new_user_listener = warp::path("channel")
        .and(warp::post())
        .and(ids_filter.clone())
        .map(move |id_list: WebhookList| {
            let rx = id_list.get_new_user_listner();
            let stream = BroadcastStream::new(rx);

            let event_stream = stream.map(move |data| match data {
                Ok(channel) => sse_counter(channel),
                Err(err) => sse_counter(err.to_string()),
            });

            warp::sse::reply(event_stream)
        });

    let new_connecion = warp::path("new_connection")
        .and(warp::path::end())
        .and(ids_filter.clone())
        .and(json_string())
        .and_then(new_connecion);

    let send_to_data = warp::post()
        .and(warp::path!("chat" / "send"))
        .and(warp::path::end())
        .and(json_message())
        .and(ids_filter.clone())
        .and_then(send);

    let get_id = warp::post()
        .and(warp::path("new_channel"))
        .and(warp::path::end())
        .and(ids_filter.clone())
        .and(json_string())
        .and_then(issue_id);

    let route = send_to_data.or(listen).or(get_id).or(new_channel_listener).or(new_connecion).or(new_user_listener);

    warp::serve(route).run(([127, 0, 0, 1], 8080)).await;
}
