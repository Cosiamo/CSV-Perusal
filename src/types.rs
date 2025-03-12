pub(crate) struct ByteString {
    pub(crate) bytestring: String,
}

pub(crate) struct Bytes<'slice> {
    pub(crate) bytes: &'slice [u8],
}