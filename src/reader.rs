use crate::context::Context;
use crate::File;
use csv::Reader;

pub struct CsvReader {
    pub delimiter: u8,
    pub inner_delimiter: u8,
    pub has_headers: bool,
    pub reader: Reader<File>

}

// Refactor this
pub trait ToPolygonCsv {
    fn parse_polygon_csv(self, context: &Context) -> CsvReader;
}

impl ToPolygonCsv for std::fs::File {
    fn parse_polygon_csv(self, context: &Context) -> CsvReader {
        let rdr = CsvReader{
            delimiter: context.delimiter,
            inner_delimiter: context.inner_delimiter,
            has_headers: context.has_headers,
            reader: csv::ReaderBuilder::new().has_headers(context.has_headers).delimiter(context.delimiter).from_reader(self)
        };
        rdr
    }
}