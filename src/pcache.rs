use std::collections::HashMap;
use std::fs::{File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, RwLock};
use std::thread::{spawn, JoinHandle};
use std::time::Instant;

use fileinfo::FileInfo;
use result::Result;

type HashTable = HashMap<PathBuf, FileInfo>;
type HashHandle = Arc<RwLock<HashTable>>;

// TODO: This file is in desperate need of some cleanup and error handling.
// (Too many unwraps().)

#[derive(Debug, Default)]
pub struct PersistedCache {
    cache: HashHandle,

    listen_handle: Option<JoinHandle<()>>,
    save_handle: Option<JoinHandle<()>>,
}

impl PersistedCache {
    // Create an empty PersistedCache.
    pub fn new() -> PersistedCache {
        PersistedCache::default()
    }

    // Load a PersistedCache from the specified filename.
    pub fn load(filename: &Path) -> Result<PersistedCache> {
        let file = File::open(filename)?;
        let hash = Self::read_hash(file)?;
        Ok(PersistedCache {
            cache: Arc::new(RwLock::new(hash)),
            ..PersistedCache::default()
        })
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
                    Self::write_hash_to_file("ff", &cache2);
                }
            }
            Self::write_hash_to_file("ff", &cache2);
        });

        self.listen_handle = Some(handle);
        self.save_handle = Some(save_handle);
    }

    fn read_hash<T>(rdr: T) -> Result<HashTable>
        where T: Read
    {
        let hash_table = serde_yaml::from_reader(rdr)?;
        Ok(hash_table)
    }

    fn write_hash_to_file<T>(filename: T, handle: &HashHandle) -> Result<()>
        where T: AsRef<Path>
    {
        // This fails if handle is poisoned. That would be a programmer error, so
        // we want to panic.
        let hashmap = &*handle.read().unwrap();
        
        let f = File::create(filename)?;
        serde_yaml::to_writer(f, hashmap).unwrap();
        Ok(())
    }

    pub fn join(self) -> HashMap<PathBuf, FileInfo> {
        self.listen_handle.map(|lh| lh.join());
        self.save_handle.map(|sh| sh.join());

        Arc::try_unwrap(self.cache).unwrap().into_inner().unwrap()
    }
}
