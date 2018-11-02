use console::Term;
use indicatif::{ProgressBar, ProgressDrawTarget};

pub enum Progrs {
    Null,
    Wrapped(ProgressBar),
}

impl Progrs {
    pub fn new(num_items: u64) -> Progrs {
        let bar = ProgressBar::new(num_items);
        bar.set_draw_target(ProgressDrawTarget::to_term(
            Term::buffered_stderr(),
            Some(2),
        ));

        Progrs::Wrapped(bar)
    }

    pub fn inc(&self) {
        self.inc_by(1);
    }

    pub fn inc_by(&self, num: u64) {
        self.with_bar(|bar| bar.inc(num));
    }

    pub fn finish(&self) {
        self.with_bar(|bar| bar.finish());
    }

    fn with_bar<T: Fn(&ProgressBar)>(&self, f: T) {
        if let Progrs::Wrapped(bar) = self {
            f(bar);
        }
    }
}

impl Default for Progrs {
    fn default() -> Progrs {
        Progrs::Null
    }
}
