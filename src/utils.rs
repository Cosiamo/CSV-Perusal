use crate::csvtype::ByteString;

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
}

// all num matches
impl ByteString {

    pub fn is_date(&self) -> bool {
        let chars = self.s.trim().chars().map(|x| x).collect::<Vec<char>>();

        if chars.contains(&"/".chars().map(|x| x).collect::<Vec<char>>()[0])
        || chars.contains(&"-".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else 
        { false }
    }

    pub fn is_time(&self) -> bool {
        let chars = self.s.trim().chars().map(|x| x).collect::<Vec<char>>();
        
        if chars.contains(&":".chars().map(|x|x).collect::<Vec<char>>()[0])
        { true }
        // else if chars.contains(&":".chars().map(|x|x).collect::<Vec<char>>()[0])
        // && chars.contains(&".".chars().map(|x|x).collect::<Vec<char>>()[0])
        // { true }
        else 
        { false }
    }

    pub fn is_datetime(&self) -> bool {
        let chars = self.s.trim().chars().map(|x| x).collect::<Vec<char>>();

        if chars.contains(&":".chars().map(|x|x).collect::<Vec<char>>()[0])
        && chars.contains(&"/".chars().map(|x| x).collect::<Vec<char>>()[0])
        || chars.contains(&"-".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true }
        else if chars.contains(&":".chars().map(|x|x).collect::<Vec<char>>()[0])
        && chars.contains(&"/".chars().map(|x| x).collect::<Vec<char>>()[0])
        || chars.contains(&"-".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars[chars.len() - 2].to_ascii_uppercase() == "A".chars().map(|x| x).collect::<Vec<char>>()[0]
        && chars[chars.len() - 1].to_ascii_uppercase() == "M".chars().map(|x| x).collect::<Vec<char>>()[0]
        { true } 
        else if chars.contains(&":".chars().map(|x|x).collect::<Vec<char>>()[0])
        && chars.contains(&"/".chars().map(|x| x).collect::<Vec<char>>()[0])
        || chars.contains(&"-".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars[chars.len() - 2].to_ascii_uppercase() == "P".chars().map(|x| x).collect::<Vec<char>>()[0]
        && chars[chars.len() - 1].to_ascii_uppercase() == "M".chars().map(|x| x).collect::<Vec<char>>()[0]
        { true } 
        else 
        { false }
    }

    pub fn is_percent_pos(&self) -> bool {
        let chars = self.s.trim().chars().map(|x| x).collect::<Vec<char>>();
        
        if chars.contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else if chars.contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0])
        && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else 
        { false }
    }

    pub fn is_percent_neg(&self) -> bool {
        let chars = self.s.trim().chars().map(|x| x).collect::<Vec<char>>();
        
        if chars.contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0])
        && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        { true } 
        else if chars.contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0])
        && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else 
        { false }
    }

    pub fn is_currency_pos(&self) -> bool {
        let chars = self.s.trim().chars().map(|x| x).collect::<Vec<char>>();
        
        // $
        if chars[0] == "$".chars().map(|x| x).collect::<Vec<char>>()[0] 
        { true } 
        else if chars[0] == "$".chars().map(|x| x).collect::<Vec<char>>()[0] 
        && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // €
        else if chars.contains(&"€".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true }
        else if chars.contains(&"€".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true }
        // £
        else if chars.contains(&"£".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else if chars.contains(&"£".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // ¥
        else if chars.contains(&"¥".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else if chars.contains(&"¥".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // ₣
        else if chars.contains(&"₣".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else if chars.contains(&"₣".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // ₹
        else if chars.contains(&"₹".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else if chars.contains(&"₹".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // د.ك
        else if chars.contains(&"د.ك".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else if chars.contains(&"د.ك".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else 
        { false }
    }

    pub fn is_currency_neg(&self) -> bool {
        let chars = self.s.trim().chars().map(|x| x).collect::<Vec<char>>();
        
        // $
        if chars[0] == "$".chars().map(|x| x).collect::<Vec<char>>()[0] 
        && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        { true } 
        else if chars[0] == "$".chars().map(|x| x).collect::<Vec<char>>()[0] 
        && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // €
        else if chars.contains(&"€".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        { true } 
        else if chars.contains(&"€".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // £
        else if chars.contains(&"£".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        { true } 
        else if chars.contains(&"£".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // ¥
        else if chars.contains(&"¥".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        { true } 
        else if chars.contains(&"¥".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // ₣
        else if chars.contains(&"₣".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        { true } 
        else if chars.contains(&"₣".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // ₹
        else if chars.contains(&"₹".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        { true } 
        else if chars.contains(&"₹".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // د.ك
        else if chars.contains(&"د.ك".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        { true } 
        else if chars.contains(&"د.ك".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else 
        { false }
    }

    pub fn is_int_neg(&self) -> bool {
        if self.s.trim().chars().map(|x| x).collect::<Vec<char>>()[0] 
        == "-".chars().map(|x| x).collect::<Vec<char>>()[0] 
        { true } 
        else 
        { false }
    }

}

// false num catch matches
impl ByteString {

    pub fn is_time_12_h(&self) -> bool {
        let chars = self.s.trim().chars().map(|x| x).collect::<Vec<char>>();

        if chars[chars.len() - 2].to_ascii_uppercase() == "A".chars().map(|x| x).collect::<Vec<char>>()[0]
        && chars[chars.len() - 1].to_ascii_uppercase() == "M".chars().map(|x| x).collect::<Vec<char>>()[0]
        { true } 
        else if chars[chars.len() - 2].to_ascii_uppercase() == "P".chars().map(|x| x).collect::<Vec<char>>()[0]
        && chars[chars.len() - 1].to_ascii_uppercase() == "M".chars().map(|x| x).collect::<Vec<char>>()[0]
        { true } 
        else 
        { false }
    }

}