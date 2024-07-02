use crate::{types::{Byte, ByteString}, CSVType};
use memchr::memmem::Finder;

impl<'slice> Byte<'slice> {
    pub fn is_dt(&self) -> bool {
        let haystack = &self.byte;
        Finder::new("/").find(haystack).is_some()
        || Finder::new("-").find(haystack).is_some()
        || Finder::new(":").find(haystack).is_some()
    }

    pub fn date_and_time(&self) -> CSVType {
        let bytestring = ByteString {bytestring: String::from_utf8_lossy(&self.byte).replace(|c: char| !c.is_ascii(), "")};
        match self {
            by if by.is_time_24h() || by.is_time_12h() => return bytestring.time_match(),
            by if by.is_datetime() => return bytestring.datetime_match(),
            by if by.is_date() => return bytestring.date_match(),
            _ => return self.catch(),
        }
    }

    pub fn am_pm(&self) -> bool {
        let haystack = &self.byte.to_ascii_uppercase();
        Finder::new("AM").find(haystack).is_some()
        || Finder::new("PM").find(haystack).is_some()
    }
}