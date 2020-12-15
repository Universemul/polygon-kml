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
    fn parse_polygon_csv(self, delimiter: u8, inner_delimiter: u8, has_headers: bool) -> CsvReader;
}

impl ToPolygonCsv for std::fs::File {
    fn parse_polygon_csv(self, delimiter: u8, inner_delimiter: u8, has_headers: bool) -> CsvReader {
        let rdr = CsvReader{
            delimiter: delimiter,
            inner_delimiter: inner_delimiter,
            has_headers: has_headers,
            reader: csv::ReaderBuilder::new().has_headers(has_headers).delimiter(delimiter).from_reader(self)
        };
        rdr
    }
}