use crate::csvtype::ByteString;
use regex::Regex;

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
}

// all num matches
impl ByteString {

    pub fn is_date(&self) -> bool {
        let date_regex = Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2}$").unwrap();
        match date_regex.captures(self.s.replace("-", "/").trim()) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn is_time(&self) -> bool {
        let hours_mins = Regex::new(r"^([01]?[0-9]|2[0-3]):[0-5][0-9]$").unwrap();
        match hours_mins.captures(self.s.trim()) {
            Some(_) => true,
            None => {
                let hours_mins_secs = Regex::new(r"^([01]?[0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$").unwrap();
                match hours_mins_secs.captures(self.s.trim()) {
                    Some(_) => true,
                    None => false,
                }
            },
        }
    }

    pub fn is_percent_pos(&self) -> bool {
        match self.s.trim().chars().map(|x| x).collect::<Vec<char>>() {
            chars if chars.contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0]) 
            => true,
            chars if chars.contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0])
            && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
            => true,
            _ => false,
        }
    }

    pub fn is_percent_neg(&self) -> bool {
        match self.s.trim().chars().map(|x| x).collect::<Vec<char>>() {
            chars if chars.contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0])
            && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
            => true,
            chars if chars.contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0])
            && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
            && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
            => true,
            _ => false,
        }
    }

    pub fn is_currency_pos(&self) -> bool {
        match self.s.trim().chars().map(|x| x).collect::<Vec<char>>() {
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
    }

    pub fn is_currency_neg(&self) -> bool {
        match self.s.trim().chars().map(|x| x).collect::<Vec<char>>() {
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
    }

}

// false num catch matches
impl ByteString {

    pub fn is_time_12h(&self) -> bool {
        let hours_mins = Regex::new(r"^([1-9]|0[1-9]|1[0-2]):[0-5][0-9] ([AaPp][Mm])$").unwrap();
        let hours_mins_secs = Regex::new(r"^([1-9]|0[1-9]|1[0-2]):[0-5][0-9]:[0-5][0-9] ([AaPp][Mm])$").unwrap();

        match self.s.trim() {
            t if hours_mins.captures(t).is_some()
            => true,
            t if hours_mins_secs.captures(t).is_some()
            => true,
            _ => false,
        }
    }

    pub fn is_datetime(&self) -> bool {
        let h24 = Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2} ([01]?[0-9]|2[0-3]):[0-5][0-9]$").unwrap();
        let h24_w_sec = Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2} ([01]?[0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$").unwrap();
        let h12 = Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2} ([1-9]|0[1-9]|1[0-2]):[0-5][0-9] ([AaPp][Mm])$").unwrap();
        let h12_w_sec = Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2} ([1-9]|0[1-9]|1[0-2]):[0-5][0-9]:[0-5][0-9] ([AaPp][Mm])$").unwrap();

        match self.s.trim() {
            dt if h24.captures(dt).is_some()
            => true,
            dt if h24_w_sec.captures(dt).is_some()
            => true,
            dt if h12.captures(dt).is_some() 
            => true,
            dt if h12_w_sec.captures(dt).is_some()
            => true,
            _ => false,
        }
    }

}