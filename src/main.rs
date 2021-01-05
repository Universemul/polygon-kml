mod reader;
mod context;
mod models;
mod utils;

extern crate getopts;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use] extern crate lazy_static;

use std::fs::File;

use context::get_args;
use utils::find_polygon;
use reader::ToPolygonCsv;

#[derive(Debug, Deserialize)]
pub struct Row {
    city: String,
    coordinates: String,
}

// GET the key in coordinates column. Everything before the first (
fn find_key(line: String, index: usize) -> String {
    match line.find('(') {
        None => panic!("Unable to find the key before the first ( at line {}", index),
        Some(position) => line[0..position].to_string()
    }
}

fn main() {
    let context = get_args();
    // Error handling here
    let mut rdr = File::open(context.filepath.clone()).unwrap().parse_polygon_csv(&context);
    for (pos, result) in rdr.reader.deserialize::<Row>().enumerate() {
        let row: Row = result.unwrap();
        let key = find_key(row.coordinates.clone(), pos + 1);
        let coordinates = find_polygon(row.coordinates.as_str());
        println!("{:?}", coordinates);
        println!("{}", key);
        //TODO: We have a list of "lat lon". We need to serialize that into Coordinate model
        // And then, we can create our Polygon model to store the city and the list of coordinates
    }
}
