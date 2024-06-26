pub(crate) struct ByteString {
    pub(crate) s: String,
}

pub(crate) struct Byte<'slice> {
    pub(crate) b: &'slice [u8],
}