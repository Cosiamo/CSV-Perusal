pub(crate) struct ByteString {
    pub(crate) bytestring: String,
}

pub(crate) struct Byte<'slice> {
    pub(crate) byte: &'slice [u8],
}