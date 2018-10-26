extern crate clap;

use Result;

#[derive(Default)]
pub struct Config {
}

impl Config {
    fn new() -> Result<Config> {
        Ok(Config { .. Config::default() })
    }
       
}
    
    // let matches = App::new("itools")
    //     .version("0.1.0")
    //     .author("George Madrid <gmadrid@gmail.com>")
    //     .about("Collection of image processing tools")
    //     .arg(Arg::with_name("files")
    //          .multiple(true)
    //          .takes_value(true)
    //          .required(true))
    //     .get_matches();
