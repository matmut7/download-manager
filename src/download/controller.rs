use tokio::sync::mpsc;
use url::Url;

use super::worker::Worker;

#[derive(Debug)]
pub enum Message {
    Downloaded(u64, u64),
    Total(u64, u64),
    Paused(u64, bool),
    Done(u64),
    Speed(u64, u64),
}

#[derive(Debug)]
pub struct Controller {
    pub workers: Vec<Worker>,
    pub rx: mpsc::UnboundedReceiver<Message>,
    pub tx: mpsc::UnboundedSender<Message>,
    next_worker_id: u64,
}

impl Controller {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<Message>();
        Self {
            workers: Vec::new(),
            next_worker_id: 0,
            tx,
            rx,
        }
    }

    pub fn download(&mut self, url: Url) {
        let _tx = self.tx.clone();
        self.workers
            .push(Worker::new(self.next_worker_id, url, _tx));
        self.next_worker_id += 1;
    }

    pub fn get_worker(&mut self, id: u64) -> &mut Worker {
        self.workers
            .iter_mut()
            .find(|worker| worker.id == id)
            .unwrap()
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}
