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

        if let Some(time) = bytestring.time_match() {
            return time;
        } else if let Some(datetime) = bytestring.datetime_match() {
            return datetime
        } else if let Some(date) = bytestring.date_match() {
            return date
        } else {
            return self.catch()
        }
    }
}