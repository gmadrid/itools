use super::search::Matches;

pub fn new_text_output() -> DynamicOutput {
    DynamicOutput::Text(TextOutput::default())
}

pub trait Output {
    fn output(&self, matches: Vec<Matches>);
}

pub enum DynamicOutput {
    Text(TextOutput),
}

impl Output for DynamicOutput {
    fn output(&self, matches: Vec<Matches>) {
        use self::DynamicOutput::*;
        match self {
            Text(to) => to.output(matches),
        };
    }
}

#[derive(Default)]
pub struct TextOutput();

impl Output for TextOutput {
    fn output(&self, matches: Vec<Matches>) {
        for mtch in matches {
            let filename = mtch.filename;
            println!("{}", filename.to_string_lossy());

            let matched_files = mtch
                .matched_files
                .into_iter()
                .filter(|fnm| *fnm != filename);
            for mf in matched_files {
                println!("   {}", mf.to_string_lossy());
            }
        }
    }
}
