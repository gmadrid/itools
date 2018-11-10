use std::sync::mpsc::{Sender, SyncSender};
use std::thread;

pub fn bool_to_option<T, F: FnOnce() -> T>(b: bool, f: F) -> Option<T> {
    if b {
        Some(f())
    } else {
        None
    }
}

pub fn spawn_with_name<T, U, F>(name: T, f: F) -> std::thread::JoinHandle<U>
where
    T: Into<String>,
    F: FnOnce() -> U + Send + 'static,
    U: Send + 'static,
{
    // TODO: improve error handling here.
    std::thread::Builder::new()
        .name(name.into())
        .spawn(f)
        .unwrap()
}

pub trait SafeSend<T> {
    fn safe_send(&self, payload: T)
    where
        T: Send;
}

impl<T> SafeSend<T> for Sender<T> {
    fn safe_send(&self, payload: T)
    where
        T: Send,
    {
        match self.send(payload) {
            Ok(_) => (),
            Err(err) => println!("Error while sending from thread \"{}\": {:?}",
                                 thread::current().name().unwrap_or("<unnamed>"), err),
        }
    }
}

impl<T> SafeSend<T> for SyncSender<T> {
    fn safe_send(&self, payload: T)
    where
        T: Send,
    {
        match self.send(payload) {
            Ok(_) => (),
            Err(err) => println!("Error while sending from thread \"{}\": {:?}",
                                 thread::current().name().unwrap_or("<unnamed>"), err),
        }
    }
}

#[cfg(test)]
mod test {
    use super::bool_to_option;

    #[test]
    fn test_bool_to_option_false() {
        assert_eq!(None, bool_to_option(false, || 5))
    }

    #[test]
    fn test_bool_to_option_true() {
        assert_eq!(Some(5), bool_to_option(true, || 5))
    }

    #[test]
    fn test_bool_to_option_closure_not_called_on_false() {
        let mut called = false;

        bool_to_option(false, || {
            called = true;
            5
        });

        assert_eq!(false, called);
    }
}
