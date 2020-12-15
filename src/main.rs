mod reader;
mod context;

extern crate getopts;

use std::fs::File;

use context::get_args;
use reader::ToPolygonCsv;

fn main() {
    let context = get_args();
    // Error handling here
    let mut rdr = File::open(context.filepath).unwrap().parse_polygon_csv(context.delimiter, context.inner_delimiter, context.has_headers);

    for result in rdr.reader.records() {
        let line = result.unwrap();
        println!("{:?}", line);
    }
}
