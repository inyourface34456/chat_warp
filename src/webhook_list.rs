use crate::{to_outer, Outer};
use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use tokio::sync::broadcast::{self, Receiver, Sender};

#[derive(Clone)]
pub struct WebhookList {
    ids: Outer<HashMap<String, Sender<String>>>,
    new_channel_listener: Sender<String>,
    new_user_listner: Sender<String>,
    user_list: Outer<Vec<String>>,
}

impl WebhookList {
    pub fn new() -> Self {
        Self {
            ids: to_outer(HashMap::new()),
            new_channel_listener: broadcast::channel(16).0,
            new_user_listner: broadcast::channel(16).0,
            user_list: to_outer(vec![]),
        }
    }

    fn from_list(ids: Outer<HashMap<String, Sender<String>>>) -> Self {
        Self {
            ids,
            ..Self::new()
        }
    }

    pub fn load(path: String) -> Self {
        if fs::metadata(&path).is_err() {
            Self::new()
        } else {
            match fs::File::open(&path) {
                Ok(mut dat) => {
                    let mut buf = String::new();
                    match dat.read_to_string(&mut buf) {
                        Ok(_) => {
                            let ids: Vec<String> = buf.split('\n').map(|x| x.into()).collect();
                            let mut map = HashMap::new();
                            for i in ids {
                                map.insert(i, broadcast::channel(16).0);
                            }
                            Self::from_list(to_outer(map))
                        }
                        Err(_) => {
                            eprintln!("error reading ids.txt");
                            Self::new()
                        }
                    }
                }
                Err(_) => {
                    let _ = fs::File::create(&path);
                    Self::new()
                }
            }
        }
    }

    pub fn issue_id(&self, name: String) -> String {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(128)
            .map(char::from)
            .collect();

        if let Ok(mut ids) = self.ids.write() {
            let (tx, _rx) = broadcast::channel(16);

            ids.insert(s.clone(), tx);
        }

        let _ = self
            .new_channel_listener
            .send(format!("new channel: {}:{}", name, s));

        s
    }

    pub fn get_id(&self, id: String) -> Option<(Sender<String>, Receiver<String>)> {
        if let Ok(ids) = self.ids.read() {
            match ids.get(&id) {
                Some(dat) => Some((dat.clone(), dat.subscribe())),
                None => None,
            }
        } else {
            None
        }
    }

    pub fn get_new_channel_listener(&self) -> Receiver<String> {
        self.new_channel_listener.subscribe()
    }

    pub fn get_new_user_listner(&self) -> Receiver<String> {
        self.new_user_listner.subscribe()
    }

    pub fn anounce_new_user(&self, username: String) {
        if let Ok(mut users) = self.user_list.write() {
            users.push(username.to_string())
        }

        let _ = self.new_user_listner.send(username);
    }
}
