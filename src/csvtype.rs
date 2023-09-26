pub enum CSVType {
    Int(i64),
    Float(f64),
    String(String),
    Date(String),
    Error(CSVTypeError),
    Empty,
}

#[derive(Debug)]
pub enum CSVTypeError {
    Parse(std::convert::Infallible),
    ByteError(csv::Error),
}
