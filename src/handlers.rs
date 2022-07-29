use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use log::{debug, error, trace};

#[derive(Clone)]
pub(crate) struct Message {
    pub(crate) body: String,
    pub(crate) endpoint: String
}

pub(crate) type SharedVector<T> = Arc<Mutex<Vec<T>>>;

pub(crate) fn start() -> SharedVector<Message> {
    let vector: SharedVector<Message> = Arc::new(Mutex::new(vec![]));
    let _vector = vector.clone();
    thread::spawn(move || {
        let client = reqwest::blocking::Client::new();

        loop {
            thread::sleep(Duration::from_secs(1));
            let mut owned_vector = vector.lock().unwrap();
            debug!("Vector elements: {}", owned_vector.len());
            let cloned_vector = owned_vector.to_owned();

            owned_vector.clear();

            for message in cloned_vector {
                let endpoint = message.endpoint.to_owned();

                match client.post(endpoint.to_owned()).body(message.body).send() {
                    Ok(_) => trace!("Successfully delivered message to {}", endpoint),
                    Err(e) => error!("Error when delivering message to {}: {:?}", endpoint, e)
                }
            }
        }
    });

    _vector
}
