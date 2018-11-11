use super::search::Matches;

pub fn output_matches(matches: Vec<Matches>) {
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

pub trait Output {
    fn output(matches: Matches);
}
