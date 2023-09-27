pub enum CSVType {
    Int(i64),
    Float(f64),
    String(String),
    Date(String),
    Time(String),
    DateTime(String),
    Error(std::convert::Infallible),
    Empty,
}
pub struct ByteString {
    pub(crate) s: String,
}