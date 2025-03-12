use crate::{types::{Bytes, ByteString}, CSVType};
use memchr::memmem::Finder;

impl<'slice> Bytes<'slice> {
    pub fn is_dt(&self) -> bool {
        let haystack = &self.bytes;
        Finder::new("/").find(haystack).is_some()
        || Finder::new("-").find(haystack).is_some()
        || Finder::new(":").find(haystack).is_some()
    }
}

impl ByteString {
    pub fn date_and_time(&self) -> CSVType {
        if let Some(time) = self.time_match() {
            return time;
        } else if let Some(datetime) = self.datetime_match() {
            return datetime
        } else if let Some(date) = self.date_match() {
            return date
        } else {
            return self.catch()
        }
    }
}