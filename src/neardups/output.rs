use super::search::Matches;

pub fn new_no_output() -> DynamicOutput {
    DynamicOutput::None(NoOutput::default())
}

pub fn new_open_output() -> DynamicOutput {
    DynamicOutput::Open(OpenOutput::default())
}

pub fn new_text_output() -> DynamicOutput {
    DynamicOutput::Text(TextOutput::default())
}

pub fn new_yaml_output() -> DynamicOutput {
    DynamicOutput::Yaml(YamlOutput::default())
}

pub trait Output {
    fn output(&self, matches: Vec<Matches>);
}

#[derive(Debug)]
pub enum DynamicOutput {
    None(NoOutput),
    Open(OpenOutput),
    Text(TextOutput),
    Yaml(YamlOutput),
}

impl Output for DynamicOutput {
    fn output(&self, matches: Vec<Matches>) {
        use self::DynamicOutput::*;
        match self {
            None(no) => no.output(matches),
            Open(oo) => oo.output(matches),
            Text(to) => to.output(matches),
            Yaml(yo) => yo.output(matches),
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

#[derive(Debug, Default)]
pub struct OpenOutput();

impl Output for OpenOutput {
    fn output(&self, _matches: Vec<Matches>) {
        panic!("WOTCHA!");
    }
}

#[derive(Debug, Default)]
pub struct YamlOutput();

impl Output for YamlOutput {
    fn output(&self, matches: Vec<Matches>) {
        serde_yaml::to_writer(std::io::stdout(), &matches).unwrap();
    }
}
