pub fn bool_to_option<T, F: FnOnce() -> T>(b: bool, f: F) -> Option<T> {
    if b {
        Some(f())
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use bool_to_option;

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
