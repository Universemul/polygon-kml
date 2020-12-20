use regex::Regex;

//static MULTIPOLYGON_REGEX: &str = r"([\-0-9\.\s,]+\s*[\-0-9\.]+,?)";
static POLYGON_REGEX: &str = r"(-?\d+\.?\d* -?\d+\.?\d*)";


pub fn find_polygon(text: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(POLYGON_REGEX).unwrap();
    }
    RE.captures_iter(text).map(|cap| cap.get(1).unwrap().as_str()).collect()
}