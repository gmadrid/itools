use console::Term;
use indicatif::{ProgressBar, ProgressDrawTarget};

type Progrs = Option<ProgressBar>;

pub fn new_counter(num_items: u64) -> Progrs {
    let bar = ProgressBar::new(num_items);
    bar.set_draw_target(ProgressDrawTarget::to_term(
        Term::buffered_stderr(),
        Some(2),
    ));
    Some(bar)
}

pub trait Progress {
    fn inc(&self);
}

fn with_bar<T: Fn(&ProgressBar)>(opt: &Progrs, f: T) {
    opt.as_ref().and_then(|b| {
        f(b);
        Some(())
    });
}

impl Progress for Progrs {
    fn inc(&self) {
        with_bar(self, |bar| bar.inc(1));
    }
}
