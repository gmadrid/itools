use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, SyncSender};
use std::sync::{Arc, RwLock};
use std::thread::{self, spawn, JoinHandle};
use std::time::Instant;

use fileinfo::FileInfo;

#[derive(Debug, Default)]
pub struct PersistedCache {
    cache: Arc<RwLock<HashMap<PathBuf, FileInfo>>>,

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
                let key = fi.filename.clone();
                cache
                    .write()
                    .unwrap()
                    .insert(key, fi);
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
                    fs::write("test_save_file.yaml", serde_yaml::to_string(hashmap).unwrap());
                }
            }
            let hashmap = &*cache2.read().unwrap();
            fs::write("test_save_file.yaml", serde_yaml::to_string(hashmap).unwrap());
        });

        self.listen_handle = Some(handle);
        self.save_handle = Some(save_handle);
    }

    pub fn join(self) -> HashMap<PathBuf, FileInfo> {
        self.listen_handle.map(|lh| lh.join());
        self.save_handle.map(|sh| sh.join());

        Arc::try_unwrap(self.cache).unwrap().into_inner().unwrap()
    }
}
