use std::io::Read;

use indicatif::ProgressBar;

pub struct SpinnerReader<T>
where
    T: Read,
{
    reader: T,
    progress_bar: ProgressBar,
}

impl<T> SpinnerReader<T>
where
    T: Read,
{
    pub fn new(reader: T, msg: &str) -> SpinnerReader<T> {
        let progress_bar = ProgressBar::new_spinner();
        progress_bar.set_message(msg);
        SpinnerReader {
            reader,
            progress_bar,
        }
    }
}

impl<T> Read for SpinnerReader<T>
where
    T: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.progress_bar.inc(1);
        self.reader.read(buf)
    }
}
