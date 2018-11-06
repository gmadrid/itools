use std::path::PathBuf;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub filename: PathBuf,
    pub a_hash: Option<String>,
    pub d_hash: Option<String>,
    pub p_hash: Option<String>,
    pub sha2_hash: Option<String>,
}
