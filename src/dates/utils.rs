use crate::{Byte, ByteString, CSVType};
use memchr::memmem::Finder;

impl<'slice> Byte<'slice> {
    pub fn is_dt(&self) -> bool {
        Finder::new("/").find(&self.b).is_some()
        || Finder::new("-").find(&self.b).is_some()
        || Finder::new(":").find(&self.b).is_some()
    }

    pub fn date_and_time(&self) -> CSVType {
        let bytestring = ByteString {s: String::from_utf8_lossy(&self.b).replace(|c: char| !c.is_ascii(), "")};
        match self {
            by if by.is_time_24h() => return bytestring.time_match(),
            by if by.is_time_12h() => return bytestring.time_match(),
            by if by.is_datetime() => return bytestring.datetime_match(),
            by if by.is_date() => return bytestring.date_match(),
            _ => return self.catch(),
        }
    }

    pub fn am_pm(&self) -> bool {
        Finder::new("AM").find(&self.b.to_ascii_uppercase()).is_some()
        || Finder::new("PM").find(&self.b.to_ascii_uppercase()).is_some()
    }
}