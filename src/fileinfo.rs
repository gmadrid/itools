use std::path::PathBuf;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub filename: PathBuf,
    pub a_hash: Option<String>,
    pub d_hash: Option<String>,
    pub p_hash: Option<String>,
    pub sha2_hash: Option<String>,
}

impl FileInfo {
    pub fn with_name<T>(name: T) -> FileInfo
    where
        T: Into<PathBuf>,
    {
        FileInfo {
            filename: name.into(),
            ..FileInfo::default()
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use FileInfo;

    #[test]
    fn test_with_name() {
        let name = "/foobar/baz";
        let fi = FileInfo::with_name(name);
        assert_eq!(PathBuf::from(name), fi.filename);
        assert_eq!(None, fi.a_hash);
        assert_eq!(None, fi.d_hash);
        assert_eq!(None, fi.p_hash);
        assert_eq!(None, fi.sha2_hash);
    }

}
