use crate::csvtype::{ByteString, CSVType, Byte};
use memchr::memmem::Finder;

impl ByteString {
    pub fn trimmed(&self) -> String {
        return self.s.trim()
        .replace("%", "")
        .replace(",", "")
        .replace(".", "")
        .replace("-", "")
        .replace("/", "")
        .replace(":", "")
        .replace("$", "")
        .replace("€", "")
        .replace("£", "")
        .replace("¥", "")
        .replace("₣", "")
        .replace("₹", "")
        .replace("د.ك", "")
    }

    pub fn remove_symbol(&self) -> String {
        return self.s.trim()
        .replace(",", "")
        .replace("%", "")
        .replace("$", "")
        .replace("€", "")
        .replace("£", "")
        .replace("¥", "")
        .replace("₣", "")
        .replace("₹", "")
        .replace("د.ك", "")
    }

    pub fn convert_to_string(&self) -> CSVType {
        match self.s.trim().parse::<String>() {
            Ok(s) => return CSVType::String(s),
            Err(_) => return CSVType::Error("PARSING ERROR: Error parsing string".to_string()),
        }
    }
}

// all num matches
impl ByteString {
    pub fn is_percent_pos(&self) -> bool {
        match self.s.trim() {
            s if s.contains("%")
            => match s.chars().map(|x| x).collect::<Vec<char>>() {
                chars if chars.contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                => true,
                chars if chars.contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0])
                && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_percent_neg(&self) -> bool {
        match self.s.trim() {
            s if s.contains("%")
            && s.contains("-")
            => match s.chars().map(|x| x).collect::<Vec<char>>() {
                chars if chars.contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0])
                && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                => true,
                chars if chars.contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0])
                && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_currency_pos(&self) -> bool {
        match self.s.trim() {
            s if s.contains("$")
            || s.contains("€")
            || s.contains("£")
            || s.contains("¥")
            || s.contains("₣")
            || s.contains("₹")
            || s.contains("د.ك")
            => {
                match s.chars().map(|x| x).collect::<Vec<char>>() {
                    // $
                    chars if chars[0] == "$".chars().map(|x| x).collect::<Vec<char>>()[0] 
                    => true,
                    chars if chars[0] == "$".chars().map(|x| x).collect::<Vec<char>>()[0] 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    // €
                    chars if chars.contains(&"€".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    chars if chars.contains(&"€".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    // £
                    chars if chars.contains(&"£".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    chars if chars.contains(&"£".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    // ¥
                    chars if chars.contains(&"¥".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    chars if chars.contains(&"¥".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    // ₣
                    chars if chars.contains(&"₣".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    chars if chars.contains(&"₣".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    // ₹
                    chars if chars.contains(&"₹".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    chars if chars.contains(&"₹".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0])
                    => true,
                    // د.ك
                    chars if chars.contains(&"د.ك".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    chars if chars.contains(&"د.ك".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    _ => false,
                }
            },
            _ => false,
        }
    }

    pub fn is_currency_neg(&self) -> bool {
        match self.s.trim() {
            s if s.contains("-")
            && s.contains("$")
            || s.contains("€")
            || s.contains("£")
            || s.contains("¥")
            || s.contains("₣")
            || s.contains("₹")
            || s.contains("د.ك")
            => {
                match s.chars().map(|x| x).collect::<Vec<char>>() {
                    // $
                    chars if chars.contains(&"$".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    chars if chars.contains(&"$".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    // €
                    chars if chars.contains(&"€".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    chars if chars.contains(&"€".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    // £
                    chars if chars.contains(&"£".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    chars if chars.contains(&"£".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    // ¥
                    chars if chars.contains(&"¥".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0]
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    chars if chars.contains(&"¥".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    // ₣
                    chars if chars.contains(&"₣".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0]
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    chars if chars.contains(&"₣".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    // ₹
                    chars if chars.contains(&"₹".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    chars if chars.contains(&"₹".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    // د.ك
                    chars if chars.contains(&"د.ك".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    chars if chars.contains(&"د.ك".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    _ => false,
                }
            },
            _ => false,
        }
    }
}

// false num catch matches
impl ByteString {
    pub fn is_empty(&self) -> bool {
        match self.s.trim().len() {
            0 => true,
            _ => false,
        }
    }

    pub fn contains_number(&self) -> bool {
        let v = self.s.trim().chars().map(|x| 
            match x {
                x if x.is_numeric() => true,
                _ => false,
            }
        ).collect::<Vec<bool>>();

        match v {
            v if v.contains(&true) => true,
            _ => false,
        }
    }
}

// ++++++++++++++++++++++++++++++++ Either going to delete everything above or move it to it's own file ++++++++++++++++++++++++++++++++

impl<'slice> Byte<'slice> {
    pub fn is_dt(&self) -> bool {
        Finder::new("/").find(&self.b).is_some()
        || Finder::new("-").find(&self.b).is_some()
        || Finder::new(":").find(&self.b).is_some()
    }

    pub fn num_match(&self) -> CSVType {
        let t = String::from_utf8_lossy(&self.b);
        let s = t.replace(|c: char| !c.is_ascii(), "");
        match s.trim().parse::<i64>() {
            Ok(v) => CSVType::Int(v),
            Err(_) => match s.trim().parse::<f64>() {
                Ok(v) => CSVType::Float(v),
                Err(_) => self.catch()
            },
        }
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

    pub fn catch(&self) -> CSVType {
        match_catch(&self.b)
    }

    pub fn am_pm(&self) -> bool {
        Finder::new("AM").find(&self.b.to_ascii_uppercase()).is_some()
        || Finder::new("PM").find(&self.b.to_ascii_uppercase()).is_some()
    }
}

pub fn match_catch(bytes: &[u8]) -> CSVType {
    let t = String::from_utf8_lossy(&bytes);
    let s = t.replace(|c: char| !c.is_ascii(), "");
    let bytestring = ByteString {s};

    match bytestring {
        bs if bs.is_empty() => return CSVType::Empty,
        bs if !bs.contains_number()
        => return bs.convert_to_string(),
        bs if bs.trimmed().chars().all(char::is_numeric)
        => match bs {
            // checks for negative percent
            bs if bs.is_percent_neg()
            => match bs.remove_symbol().parse::<f64>() {
                Ok(v) => return CSVType::Float(v),
                Err(_) => return bs.convert_to_string(),
            },
            // checks for positive percent
            bs if bs.is_percent_pos()
            => match bs.remove_symbol().parse::<f64>() {
                Ok(v) => return CSVType::Float(v),
                Err(_) => return bs.convert_to_string(),
            },
            // checks for negative currency
            bs if bs.is_currency_neg()
            => match bs.remove_symbol().parse::<f64>() {
                Ok(v) => return CSVType::Float(v),
                Err(_) => return bs.convert_to_string(),
            },
            // checks for positive currency 
            bs if bs.is_currency_pos()
            => match bs.remove_symbol().parse::<f64>() {
                Ok(v) => return CSVType::Float(v),
                Err(_) => return bs.convert_to_string(),
            },
            _ => return bs.convert_to_string(),
        },
        bs if bs.contains_number()
        => match bs {
            bs if bs.is_date_w_abbrv() => return bs.date_w_abbrv_match(),
            _ => return bs.convert_to_string(),
        },
        _ => return bytestring.convert_to_string(),
    }
}