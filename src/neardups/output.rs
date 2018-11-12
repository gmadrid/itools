use super::search::Matches;

pub fn new_no_output() -> DynamicOutput {
    DynamicOutput::None(NoOutput::default())
}

pub fn new_text_output() -> DynamicOutput {
    DynamicOutput::Text(TextOutput::default())
}

pub trait Output {
    fn output(&self, matches: Vec<Matches>);
}

#[derive(Debug)]
pub enum DynamicOutput {
    None(NoOutput),
    Text(TextOutput),
}

impl Output for DynamicOutput {
    fn output(&self, matches: Vec<Matches>) {
        use self::DynamicOutput::*;
        match self {
            None(no) => no.output(matches),
            Text(to) => to.output(matches),
        };
    }
}

impl Default for DynamicOutput {
    fn default() -> DynamicOutput {
        DynamicOutput::Text(TextOutput::default())
    }
}

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
pub struct NoOutput();

impl Output for NoOutput {
    fn output(&self, _matches: Vec<Matches>) {}
}
