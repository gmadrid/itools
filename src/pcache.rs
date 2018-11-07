use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, SyncSender};
use std::sync::{Arc, RwLock};
use std::thread::{self, spawn, JoinHandle};
use std::time::Instant;

use fileinfo::FileInfo;

#[derive(Debug, Default)]
pub struct PersistedCache {
    cache: Arc<RwLock<HashMap<String, Vec<FileInfo>>>>,

    listen_handle: Option<JoinHandle<()>>,
    save_handle: Option<JoinHandle<()>>,
}

impl PersistedCache {
    pub fn new() -> PersistedCache {
        PersistedCache::default()
    }

    pub fn run(&mut self, rx: Receiver<FileInfo>) {
        let cache = Arc::clone(&self.cache);
        let cache2 = Arc::clone(&self.cache);

        // Send true to indicate some change that was made.
        let (ltx, lrx) = channel::<bool>();

        let handle = spawn(move || {
            for fi in rx {
                let key = fi.sha2_hash.as_ref().unwrap().clone();
                cache
                    .write()
                    .unwrap()
                    .entry(key)
                    .or_insert_with(|| Vec::default())
                    .push(fi);
                ltx.send(true).unwrap();
            }
        });

        let save_handle = spawn(move || {
            let mut last_save_time = Instant::now();
            for _ in lrx {
                // Every 5 seconds.
                let elapsed = last_save_time.elapsed();
                if elapsed.as_secs() >= 5 {
                    last_save_time = Instant::now();

                    let hashmap = &*cache2.read().unwrap();
                    println!(
                        "DDD\n{}",
                        serde_yaml::to_string(&*cache2.read().unwrap()).unwrap()
                    );
                }
            }
            // TODO: you nned to save one last time before quitting.
        });

        self.listen_handle = Some(handle);
        self.save_handle = Some(save_handle);
    }

    pub fn join(self) -> HashMap<String, Vec<FileInfo>> {
        self.listen_handle.map(|lh| lh.join());
        self.save_handle.map(|sh| sh.join());

        Arc::try_unwrap(self.cache).unwrap().into_inner().unwrap()
    }
}
