use std::path::PathBuf;

use img_hash::ImageHash;

#[derive(Default, Debug)]
pub struct FileInfo {
    pub filename: PathBuf,
    pub a_hash: Option<ImageHash>,
    pub d_hash: Option<ImageHash>,
    pub p_hash: Option<ImageHash>,
    pub sha2_hash: Option<Vec<u8>>,
}
