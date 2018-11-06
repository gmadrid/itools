use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread::{self, spawn, JoinHandle};

use fileinfo::FileInfo;

#[derive(Debug, Default)]
pub struct PersistedCache {
    cache: Arc<RwLock<HashMap<String, Vec<FileInfo>>>>,

    listen_handle: Option<JoinHandle<()>>,
}

impl PersistedCache {
    pub fn new() -> PersistedCache {
        PersistedCache::default()
    }

    pub fn run(&mut self, rx: Receiver<FileInfo>) {
        let cache = Arc::clone(&self.cache);
        let handle = thread::spawn(move || {
            for fi in rx {
                let key = fi.sha2_hash.as_ref().unwrap().clone();
                cache.write().unwrap().entry(key)
                    .or_insert_with(|| Vec::default()).push(fi);
                println!("HASH: {:?}", cache.read().as_ref());
            }
        });

        self.listen_handle = Some(handle);
    }

    pub fn join(self) -> HashMap<String, Vec<FileInfo>> {
        self.listen_handle.map(|lh| lh.join());

        Arc::try_unwrap(self.cache).unwrap().into_inner().unwrap()
    }
}
