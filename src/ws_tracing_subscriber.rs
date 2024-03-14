use std::{collections::HashMap, sync::Arc};

use fastwebsockets::{upgrade::UpgradeFut, Frame};
use tokio::sync::{mpsc, Mutex};

#[derive(Clone)]
pub struct WebSocketAppender {
    subscribers: Arc<Mutex<HashMap<uuid::Uuid, mpsc::Sender<Vec<u8>>>>>,
    tx: mpsc::Sender<Vec<u8>>,
}

impl WebSocketAppender {
    pub fn new() -> (Self, mpsc::Receiver<Vec<u8>>) {
        let (tx, rx) = mpsc::channel(100);
        (
            Self {
                subscribers: Arc::new(Mutex::new(HashMap::new())),
                tx,
            },
            rx,
        )
    }

    pub async fn add_subscriber(&self, fut: UpgradeFut) {
        let (tx, rx) = mpsc::channel(100);
        let id = uuid::Uuid::new_v4();
        eprintln!("New traces subscriber with id: {id}");

        let subscribers = self.subscribers.clone();
        // Spawn task for new web socket subscriber
        tokio::spawn(async move {
            let mut rx: mpsc::Receiver<Vec<u8>> = rx;
            let mut ws = fastwebsockets::FragmentCollector::new(
                fut.await.expect("Failed to await ws future"),
            );

            loop {
                tokio::select! {
                    log = rx.recv() => {
                        if let Some(log) = log {
                            if let Err(e) = ws.write_frame(Frame::text(fastwebsockets::Payload::Owned(log))).await {
                                // Don't use tracing here to avoid feedback
                                eprintln!("Failed to send log over ws: {e}");
                                break;
                            }
                        }
                    }
                    frame = ws.read_frame() => {
                        // Assume connection is closed
                        if frame.is_err() {
                            eprintln!("Connection is closed (tracing subscription)");
                            break;
                        }
                    }
                }
            }

            // Remove subscriber from notificator's list
            eprintln!("Removing id {id} from tracing subscribers list");
            subscribers.lock().await.remove(&id);
        });

        // Store reference to subscriber's transmitter
        self.subscribers.lock().await.insert(id, tx);
    }

    pub fn run(&self, mut rx: mpsc::Receiver<Vec<u8>>) {
        let subscribers = self.subscribers.clone();
        tokio::spawn(async move {
            loop {
                let value = match rx.recv().await {
                    Some(v) => v,
                    None => {
                        tracing::error!("Failed to send ws trace");
                        break;
                    }
                };
                // Send traces for all subscribers over web socket
                for (_, sender) in subscribers.lock().await.iter_mut() {
                    if let Err(e) = sender.send(value.clone()).await {
                        tracing::error!(
                        "Failed to send web socket trace over mpsc::channel: {e}"
                    );
                    }
                }
            }
        });
    }
}

impl std::io::Write for WebSocketAppender {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let buf_len = buf.len();
        if let Err(e) = self.tx.blocking_send(buf.to_owned()) {
            tracing::error!("Failed to send trace on std::sync::mpsc: {e}")
        }
        Ok(buf_len)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
