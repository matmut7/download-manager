use std::{collections::VecDeque, path::Path, time::Duration};

use futures::StreamExt;
use reqwest::Response;
use tokio::{io::AsyncWriteExt, sync::mpsc, time::Instant};
use url::Url;

use super::controller::Message;

#[derive(Debug)]
pub struct Worker {
    pub total_size: u64,
    pub downloaded: u64,
    pub url: Url,
    pub file_name: String,
    pub id: u64,
    pub pause_tx: mpsc::UnboundedSender<()>,
    pub paused: bool,
    pub done: bool,
    pub speed: u64,
}

impl Worker {
    pub fn new(id: u64, url: Url, tx: mpsc::UnboundedSender<Message>) -> Self {
        let file_name = url.path_segments().unwrap().last().unwrap().to_string();
        let _url = url.clone();
        let _file_name = file_name.clone();
        let (pause_tx, pause_rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            // init connection and get total size
            let response = reqwest::get(_url).await.unwrap();
            let total_size = response.content_length().unwrap();
            tx.send(Message::Total(id, total_size)).unwrap();

            // calculate file name, extension, temp name, etc.
            let file = Path::new(&_file_name);
            let file_stem = file.file_stem().unwrap().to_str().unwrap().to_string();
            let file_ext = file.extension().unwrap().to_str().unwrap().to_string();
            let tmp_file_name = format!("{}.tmp.{}", file_stem, file_ext);
            let file = tokio::fs::File::create(&tmp_file_name).await.unwrap();

            start_download_loop(response, pause_rx, &tx, file, id, total_size).await;

            tx.send(Message::Done(id)).unwrap();
            tokio::fs::rename(tmp_file_name, _file_name).await.unwrap();
        });

        Self {
            total_size: 0,
            downloaded: 0,
            speed: 0,
            paused: false,
            done: false,
            url,
            id,
            file_name,
            pause_tx,
        }
    }

    pub fn ratio(&self) -> f64 {
        if self.total_size == 0 {
            0.0
        } else if self.downloaded > self.total_size {
            1.0
        } else {
            self.downloaded as f64 / self.total_size as f64
        }
    }
}

async fn start_download_loop(
    response: Response,
    mut pause_rx: mpsc::UnboundedReceiver<()>,
    tx: &mpsc::UnboundedSender<Message>,
    mut file: tokio::fs::File,
    id: u64,
    total_size: u64,
) {
    // loop in which we receive chunks and send update messages
    let mut last_progress_message_time = Instant::now();
    let mut last_speed_message_time = Instant::now();
    let mut downloaded_last: u64 = 0;
    let mut sliding_speed = VecDeque::<u64>::from(vec![0; 3]);
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();
    let mut paused = false;
    loop {
        // do not pull next chunk if paused
        if paused {
            pause_rx.recv().await.unwrap();
            paused = !paused;
            tx.send(Message::Paused(id, paused)).unwrap();
        } else {
            // receive next data chunk or a message from controller
            tokio::select! {
                option = stream.next() => {
                    match option {
                        Some(chunk) => {
                            let chunk = chunk.unwrap();
                            file.write_all(&chunk).await.unwrap();
                            downloaded += u64::try_from(chunk.len()).unwrap();
                            if last_progress_message_time.elapsed() >= Duration::from_millis(100)
                                || downloaded == total_size
                            {
                                tx.send(Message::Downloaded(id, downloaded)).unwrap();
                                last_progress_message_time = Instant::now();
                            }
                            if last_speed_message_time.elapsed() >= Duration::from_secs(1) {
                                sliding_speed.pop_front();
                                sliding_speed.push_back(downloaded - downloaded_last);
                                tx.send(Message::Speed(id, sliding_speed.iter().sum::<u64>() / 3))
                                    .unwrap();
                                downloaded_last = downloaded;
                                last_speed_message_time = Instant::now();
                            }
                        }
                        None => break
                    }
                }
                _ = pause_rx.recv() => {
                    paused = !paused;
                    tx.send(Message::Paused(id, paused)).unwrap();
                }
            }
        }
    }
}
