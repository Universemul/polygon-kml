use std::{env, process::exit};
use getopts::Options;

pub struct Context {
    pub filepath: String,
    pub delimiter: u8,
    pub inner_delimiter: u8,
    pub has_headers: bool,
    pub output: String
}

fn print_usage(){
    println!("HELP METHOD");
    exit(0);
}

pub fn get_args() -> Context {
    let args: Vec<String> = env::args().collect();
    // Set the flags
    let mut opts = Options::new();
    opts.optopt("d", "delimiter", "Set the csv delimiter. Default is ;", "DELIMITER");
    opts.optopt("i", "inner_delimiter", "Set the delimiter between Polygon. Default is ,", "INNER_DELIMITER");
    opts.optopt("o", "output", "Set output file name", "NAME");
    opts.optflag("h", "help", "Print the help menu");
    opts.optflag("t", "headers", "Set if the csv file has headers. Default is false");
    let matches = match opts.parse(&args) {
        Ok(m) => {m},
        Err(e) => panic!(e)
    };
    if matches.opt_present("h") || matches.free.len() == 1{
        print_usage();
    }
    let filepath = matches.free.get(1).unwrap();
    let delimiter = matches.opt_get_default("d", ';').unwrap();
    let inner_delimiter = matches.opt_get_default("i", ',').unwrap();
    let output = matches.opt_get_default("o", "output.kml".to_string()).unwrap();
    let has_headers = matches.opt_present("t");
    Context{
        filepath: filepath.clone(),
        delimiter: delimiter as u8,
        inner_delimiter: inner_delimiter as u8,
        output: output.clone(),
        has_headers: has_headers
    }
}

