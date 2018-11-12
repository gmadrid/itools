use std::path::PathBuf;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub filename: PathBuf,
    pub a_hash: String,
    pub d_hash: String,
    pub p_hash: String,
    pub sha2_hash: String,
}

#[derive(Default, Debug)]
pub struct FileInfoIncomplete {
    pub filename: PathBuf,
    pub a_hash: Option<String>,
    pub d_hash: Option<String>,
    pub p_hash: Option<String>,
    pub sha2_hash: Option<String>,
}

impl FileInfoIncomplete {
    pub fn with_name<T>(name: T) -> FileInfoIncomplete
    where
        T: Into<PathBuf>,
    {
        FileInfoIncomplete {
            filename: name.into(),
            ..FileInfoIncomplete::default()
        }
    }

    pub fn is_complete(&self) -> bool {
        self.a_hash.is_some()
            && self.d_hash.is_some()
            && self.p_hash.is_some()
            && self.sha2_hash.is_some()
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::FileInfo;

    #[test]
    fn test_is_complete() {
        let fi = FileInfo::default();
        assert_eq!(false, fi.is_complete());

        let fi = FileInfo {
            a_hash: Some("sisisi".into()),
            ..fi
        };
        assert_eq!(false, fi.is_complete());

        let fi = FileInfo {
            d_hash: Some("foobar".into()),
            p_hash: Some("blah".into()),
            sha2_hash: Some("xxxxx".into()),
            ..fi
        };
        assert_eq!(true, fi.is_complete());
    }

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
