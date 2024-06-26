#[macro_export]
macro_rules! vector_to_byterecord {
    ($data:expr) => {{
        let tabbed = $data.iter().map(|row| row.join("\t")).collect::<Vec<String>>();
        let tsv = tabbed.join("\n");
        let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(tsv.as_bytes());
        let mut byterecord = Vec::new(); 
        for result in reader.byte_records() { byterecord.push(result?) }
        byterecord
    }};
}