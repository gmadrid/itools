use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, RwLock};
use std::thread::{spawn, JoinHandle};
use std::time::Instant;

use fileinfo::FileInfo;
use utils::bool_to_option;

type HashTable = HashMap<PathBuf, FileInfo>;
type HashHandle = Arc<RwLock<HashTable>>;

static DEFAULT_FILE_NAME: &str = "test_output.yaml";

// TODO: This file is in desperate need of some cleanup and error handling.
// (Too many unwraps().)

#[derive(Debug, Default)]
pub struct PersistedCache {
    cache: HashHandle,

    listen_handle: Option<JoinHandle<()>>,
    save_handle: Option<JoinHandle<()>>,
}

impl PersistedCache {
    pub fn new() -> PersistedCache {
        PersistedCache::default()
    }

    pub fn load(filename: &Path) -> PersistedCache {
        // TODO: oooooh, this is broken.
        let rdr = bool_to_option(filename.is_file(),
                                 || fs::File::open(filename).unwrap());
        if let Some(_) = rdr {
            let hash = Self::read_hash_from_file();
            PersistedCache {
                cache: Arc::new(RwLock::new(hash)),
                ..PersistedCache::default()
            }
        } else {
            PersistedCache::default()
        }
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
                    Self::write_hash_to_file(&cache2);
                }
            }
            Self::write_hash_to_file(&cache2);
        });

        self.listen_handle = Some(handle);
        self.save_handle = Some(save_handle);
    }

    fn read_hash_from_file() -> HashTable {
        let path = Path::new(DEFAULT_FILE_NAME);
        if let Some(rdr) = bool_to_option(path.is_file(),
                                          || fs::File::open(path).unwrap()) {
            serde_yaml::from_reader(rdr).unwrap()
        } else {
            HashTable::default()
        }
    }

    fn write_hash_to_file(handle: &HashHandle) {
        let hashmap = &*handle.read().unwrap();
        let f = fs::File::create(Path::new(DEFAULT_FILE_NAME)).unwrap();
        serde_yaml::to_writer(f, hashmap).unwrap();
    }

    pub fn join(self) -> HashMap<PathBuf, FileInfo> {
        self.listen_handle.map(|lh| lh.join());
        self.save_handle.map(|sh| sh.join());

        Arc::try_unwrap(self.cache).unwrap().into_inner().unwrap()
    }
}
