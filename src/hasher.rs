use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, RwLock};
use std::thread::{spawn, JoinHandle};

use img_hash::{HashType, ImageHash};
use serialize::base64::{ToBase64, STANDARD}; // , FromBase64, STANDARD};
use sha2::{Digest, Sha256};

use fileinfo::FileInfo;

#[derive(Debug)]
pub struct Hasher {
    file_reader_handle: JoinHandle<()>,
    sha2_hasher_handle: JoinHandle<()>,
    image_creator_handle: JoinHandle<()>,
    ahash_handle: JoinHandle<()>,
    dhash_handle: JoinHandle<()>,
    phash_handle: JoinHandle<()>,
    aggregator_handle: JoinHandle<()>,

    aggregator_rx: Receiver<FileInfo>,
}

impl Hasher {
    pub fn run(files: Vec<PathBuf>) -> Hasher {
        let (sha_hasher_rx, image_creator_rx, file_reader_handle) = make_file_reader(files);
        let (aggregator_tx, aggregator_rx, sha2_hasher_handle) = make_sha2_hasher(sha_hasher_rx);
        let (ahash_rx, dhash_rx, phash_rx, image_creator_handle) =
            make_image_creator(image_creator_rx);

        let ahash_handle = make_ahasher(ahash_rx, aggregator_tx.clone());
        let dhash_handle = make_dhasher(dhash_rx, aggregator_tx.clone());
        let phash_handle = make_phasher(phash_rx, aggregator_tx);
        let (aggregator_rx, aggregator_handle) = make_aggregator(aggregator_rx);

        Hasher {
            file_reader_handle,
            sha2_hasher_handle,
            image_creator_handle,
            ahash_handle,
            dhash_handle,
            phash_handle,
            aggregator_handle,

            aggregator_rx,
        }
    }

    pub fn agg_receiver(&self) -> &Receiver<FileInfo> {
        &self.aggregator_rx
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

type FileInfoHandle = Arc<RwLock<FileInfo>>;
type VecHandle<T> = Arc<Vec<T>>;
type ImageHandle = Arc<image::DynamicImage>;

fn make_file_reader(
    files: Vec<PathBuf>,
) -> (
    Receiver<(FileInfoHandle, VecHandle<u8>)>,
    Receiver<(FileInfoHandle, VecHandle<u8>)>,
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
    fi_receiver: Receiver<(FileInfoHandle, VecHandle<u8>)>,
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
                w.sha2_hash = Some(hasher.result_reset().to_vec().to_base64(STANDARD));
            }
            tx_local.send(fi).unwrap();
        }
    });

    (tx, rx, handle)
}

fn make_image_creator(
    fi_receiver: Receiver<(FileInfoHandle, VecHandle<u8>)>,
) -> (
    Receiver<(FileInfoHandle, ImageHandle)>,
    Receiver<(FileInfoHandle, ImageHandle)>,
    Receiver<(FileInfoHandle, ImageHandle)>,
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
    rx: Receiver<(FileInfoHandle, ImageHandle)>,
    tx: SyncSender<FileInfoHandle>,
) -> JoinHandle<()> {
    let handle = spawn(move || {
        for (fi, image) in rx {
            let ahash = ImageHash::hash(image.as_ref(), 8, HashType::Mean);
            {
                let mut w = fi.write().unwrap();
                w.a_hash = Some(ahash.to_base64());
            }
            tx.send(fi).unwrap();
        }
    });

    handle
}

fn make_dhasher(
    rx: Receiver<(FileInfoHandle, ImageHandle)>,
    tx: SyncSender<FileInfoHandle>,
) -> JoinHandle<()> {
    let handle = spawn(move || {
        for (fi, image) in rx {
            let ahash = ImageHash::hash(image.as_ref(), 8, HashType::Gradient);
            {
                let mut w = fi.write().unwrap();
                w.d_hash = Some(ahash.to_base64());
            }
            tx.send(fi).unwrap();
        }
    });

    handle
}

fn make_phasher(
    rx: Receiver<(FileInfoHandle, ImageHandle)>,
    tx: SyncSender<FileInfoHandle>,
) -> JoinHandle<()> {
    let handle = spawn(move || {
        for (fi, image) in rx {
            let phash = ImageHash::hash(image.as_ref(), 8, HashType::DCT);
            {
                let mut w = fi.write().unwrap();
                w.p_hash = Some(phash.to_base64());
            }
            tx.send(fi).unwrap();
        }
    });

    handle
}

fn make_aggregator(fi_rx: Receiver<FileInfoHandle>) -> (Receiver<FileInfo>, JoinHandle<()>) {
    let (tx, rx) = sync_channel(0);

    let handle = spawn(move || {
        for fi in fi_rx {
            let fi_complete;
            {
                let fi_read = fi.read().unwrap();
                fi_complete = fi_read.a_hash.is_some()
                    && fi_read.d_hash.is_some()
                    && fi_read.p_hash.is_some()
                    && fi_read.sha2_hash.is_some()
            }
            if fi_complete {
                // This mess is how you get an owned object out of an Arc<RwLock>.
                // It will only work if Arc::strong_count == 1.
                tx.send(Arc::try_unwrap(fi).unwrap().into_inner().unwrap())
                    .unwrap();
            }
        }
    });

    (rx, handle)
}
