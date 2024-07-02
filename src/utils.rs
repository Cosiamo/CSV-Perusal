/// Converts a two-dimensional vector into a vector of [`csv::ByteRecord`].
/// 
/// The macro joins the inner vector via a tab, then converts the input data into a [`String`].
/// This creates a tab-separated-value grid that is be used to build a vector of [`csv::ByteRecord`].
#[macro_export]
macro_rules! grid_to_byterecord {
    ($data:expr) => {{
        let tabbed = $data.iter().map(|row| row.join("\t")).collect::<Vec<String>>();
        let tsv = tabbed.join("\n");
        let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(tsv.as_bytes());
        let mut byterecord = Vec::new(); 
        for result in reader.byte_records() { byterecord.push(result?) }
        byterecord
    }};
}