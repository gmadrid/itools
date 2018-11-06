use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, RwLock};
use std::thread::{spawn, JoinHandle};

use img_hash::{HashType, ImageHash};
use sha2::{Digest, Sha256};

use fileinfo::FileInfo;

#[derive(Debug)]
pub struct Machine {
    file_reader_handle: JoinHandle<()>,
    sha2_hasher_handle: JoinHandle<()>,
    image_creator_handle: JoinHandle<()>,
    ahash_handle: JoinHandle<()>,
    dhash_handle: JoinHandle<()>,
    phash_handle: JoinHandle<()>,
    aggregator_handle: JoinHandle<()>,
}

type FileInfoHandle = Arc<RwLock<FileInfo>>;

fn make_file_reader(
    files: Vec<PathBuf>,
) -> (
    Receiver<(FileInfoHandle, Arc<Vec<u8>>)>,
    Receiver<(FileInfoHandle, Arc<Vec<u8>>)>,
    JoinHandle<()>,
) {
    let (tx0, rx0) = sync_channel(0);
    let (tx1, rx1) = sync_channel(1);

    let handle = spawn(move || {
        for file in files {
            let buf = fs::read(&file).unwrap();
            let fi = FileInfo {
                filename: file,
                ..FileInfo::default()
            };
            let fi_handle = Arc::new(RwLock::new(fi));
            let buf_handle = Arc::new(buf);
            tx0.send((Arc::clone(&fi_handle), Arc::clone(&buf_handle)))
                .unwrap();
            tx1.send((Arc::clone(&fi_handle), Arc::clone(&buf_handle)))
                .unwrap();
        }
    });

    (rx0, rx1, handle)
}

fn make_sha2_hasher(
    fi_receiver: Receiver<(FileInfoHandle, Arc<Vec<u8>>)>,
) -> (
    SyncSender<FileInfoHandle>,
    Receiver<FileInfoHandle>,
    JoinHandle<()>,
) {
    let (tx, rx) = sync_channel(0);

    let tx_local = tx.clone();
    let handle = spawn(move || {
        let mut hasher = Sha256::new();
        for (mut fi, buf) in fi_receiver {
            hasher.input(buf.as_ref());
            {
                let mut w = fi.write().unwrap();
                w.sha2_hash = Some(hasher.result_reset().to_vec());
            }
            tx_local.send(fi).unwrap();
        }
    });

    (tx, rx, handle)
}

fn make_image_creator(
    fi_receiver: Receiver<(FileInfoHandle, Arc<Vec<u8>>)>,
) -> (
    Receiver<(FileInfoHandle, Arc<image::DynamicImage>)>,
    Receiver<(FileInfoHandle, Arc<image::DynamicImage>)>,
    Receiver<(FileInfoHandle, Arc<image::DynamicImage>)>,
    JoinHandle<()>,
) {
    let (tx0, rx0) = sync_channel(0);
    let (tx1, rx1) = sync_channel(0);
    let (tx2, rx2) = sync_channel(0);

    let handle = spawn(move || {
        for (fi, image_buf) in fi_receiver {
            let im = image::load_from_memory(&image_buf).unwrap();
            let im_handle = Arc::new(im);
            tx0.send((Arc::clone(&fi), Arc::clone(&im_handle))).unwrap();
            tx1.send((Arc::clone(&fi), Arc::clone(&im_handle))).unwrap();
            tx2.send((fi, im_handle)).unwrap();
        }
    });

    (rx0, rx1, rx2, handle)
}

fn make_ahasher(
    rx: Receiver<(FileInfoHandle, Arc<image::DynamicImage>)>,
    tx: SyncSender<FileInfoHandle>,
) -> JoinHandle<()> {
    let handle = spawn(move || {
        for (fi, image) in rx {
            let ahash = ImageHash::hash(image.as_ref(), 8, HashType::Mean);
            {
                let mut w = fi.write().unwrap();
                w.a_hash = Some(ahash);
            }
            tx.send(fi).unwrap();
        }
    });

    handle
}

fn make_dhasher(
    rx: Receiver<(FileInfoHandle, Arc<image::DynamicImage>)>,
    tx: SyncSender<FileInfoHandle>,
) -> JoinHandle<()> {
    let handle = spawn(move || {
        for (fi, image) in rx {
            let ahash = ImageHash::hash(image.as_ref(), 8, HashType::Gradient);
            {
                let mut w = fi.write().unwrap();
                w.d_hash = Some(ahash);
            }
            tx.send(fi).unwrap();
        }
    });

    handle
}

fn make_phasher(
    rx: Receiver<(FileInfoHandle, Arc<image::DynamicImage>)>,
    tx: SyncSender<FileInfoHandle>,
) -> JoinHandle<()> {
    let handle = spawn(move || {
        for (fi, image) in rx {
            let phash = ImageHash::hash(image.as_ref(), 8, HashType::DCT);
            {
                let mut w = fi.write().unwrap();
                w.p_hash = Some(phash);
            }
            tx.send(fi).unwrap();
        }
    });

    handle
}

fn make_aggregator(rx: Receiver<FileInfoHandle>) -> JoinHandle<()> {
    let handle = spawn(move || {
        for fi in rx {
            let fi_read = fi.read().unwrap();
            if fi_read.a_hash.is_some() &&
                fi_read.d_hash.is_some() &&
                fi_read.p_hash.is_some() &&
                fi_read.sha2_hash.is_some() {
                println!("AGG: {}", fi.read().unwrap().filename.to_string_lossy());
            }
        }
    });
    handle
}

impl Machine {
    pub fn run(files: Vec<PathBuf>) -> Machine {
        let (sha_hasher_rx, image_creator_rx, file_reader_handle) = make_file_reader(files);
        let (aggregator_tx, aggregator_rx, sha2_hasher_handle) = make_sha2_hasher(sha_hasher_rx);
        let (ahash_rx, dhash_rx, phash_rx, image_creator_handle) =
            make_image_creator(image_creator_rx);

        let ahash_handle = make_ahasher(ahash_rx, aggregator_tx.clone());
        let dhash_handle = make_dhasher(dhash_rx, aggregator_tx.clone());
        let phash_handle = make_phasher(phash_rx, aggregator_tx);
        let aggregator_handle = make_aggregator(aggregator_rx);

        Machine {
            file_reader_handle,
            sha2_hasher_handle,
            image_creator_handle,
            ahash_handle,
            dhash_handle,
            phash_handle,
            aggregator_handle,
        }
    }

    pub fn join(self) {
        self.file_reader_handle.join().unwrap();
        self.sha2_hasher_handle.join().unwrap();
        self.image_creator_handle.join().unwrap();
        self.ahash_handle.join().unwrap();
        self.dhash_handle.join().unwrap();
        self.phash_handle.join().unwrap();
        self.aggregator_handle.join().unwrap();
    }
}
