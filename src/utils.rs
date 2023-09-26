use crate::csvtype::{CSVType, CSVTypeError};
pub struct ByteString {
    pub(crate) s: String,
}

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

    pub fn is_date(&self) -> bool {
        if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"/".chars().map(|x| x).collect::<Vec<char>>()[0])
        || self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"-".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else { false }
    }

    pub fn is_percent_pos(&self) -> bool {
        if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0])
        && 
        self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else { false }
    }

    pub fn is_percent_neg(&self) -> bool {
        if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0])
        && self.s.trim().chars()
            .map(|x| x).collect::<Vec<_>>()[0] 
        == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        { true } 
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0])
        && self.s.trim().chars()
            .map(|x| x).collect::<Vec<_>>()[0] 
        == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        && self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else { false }
    }

    pub fn is_currency_pos(&self) -> bool {
        // $
        if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()[0]
        == "$".chars().map(|x| x).collect::<Vec<char>>()[0] 
        { true } 
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()[0]
        == "$".chars().map(|x| x).collect::<Vec<char>>()[0] 
        && self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // €
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"€".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true }
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"€".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true }
        // £
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"£".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"£".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // ¥
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"¥".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"¥".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // ₣
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"₣".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"₣".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // ₹
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"₹".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"₹".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // د.ك
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"د.ك".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"د.ك".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else { false }
    }

    pub fn is_currency_neg(&self) -> bool {
        // $
        if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()[0]
        == "$".chars().map(|x| x).collect::<Vec<char>>()[0] 
        && self.s.trim().chars()
            .map(|x| x).collect::<Vec<_>>()[0] 
        == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        { true } 
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()[0]
        == "$".chars().map(|x| x).collect::<Vec<char>>()[0] 
        && self.s.trim().chars()
            .map(|x| x).collect::<Vec<_>>()[0] 
        == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        && self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // €
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"€".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && self.s.trim().chars()
            .map(|x| x).collect::<Vec<_>>()[0] 
        == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        { true } 
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"€".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && self.s.trim().chars()
            .map(|x| x).collect::<Vec<_>>()[0] 
        == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        && self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // £
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"£".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && self.s.trim().chars()
            .map(|x| x).collect::<Vec<_>>()[0] 
        == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        { true } 
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"£".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && self.s.trim().chars()
            .map(|x| x).collect::<Vec<_>>()[0] 
        == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        && self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // ¥
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"¥".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && self.s.trim().chars()
            .map(|x| x).collect::<Vec<_>>()[0] 
        == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        { true } 
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"¥".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && self.s.trim().chars()
            .map(|x| x).collect::<Vec<_>>()[0] 
        == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        && self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // ₣
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"₣".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && self.s.trim().chars()
            .map(|x| x).collect::<Vec<_>>()[0] 
        == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        { true } 
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"₣".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && self.s.trim().chars()
            .map(|x| x).collect::<Vec<_>>()[0] 
        == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        && self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // ₹
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"₹".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && self.s.trim().chars()
            .map(|x| x).collect::<Vec<_>>()[0] 
        == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        { true } 
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"₹".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && self.s.trim().chars()
            .map(|x| x).collect::<Vec<_>>()[0] 
        == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        && self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        // د.ك
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"د.ك".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && self.s.trim().chars()
            .map(|x| x).collect::<Vec<_>>()[0] 
        == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        { true } 
        else if self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&"د.ك".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        && self.s.trim().chars()
            .map(|x| x).collect::<Vec<_>>()[0] 
        == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
        && self.s.chars()
            .map(|x| x).collect::<Vec<char>>()
            .contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
        { true } 
        else { false }
    }

    pub fn is_int_neg(&self) -> bool {
        if self.s.trim().chars()
            .map(|x| x).collect::<Vec<char>>()[0] 
        == "-".chars().map(|x| x).collect::<Vec<char>>()[0] 
        { true  } 
        else { false }
    }
}

impl ByteString {
    pub fn date_match(&self) -> CSVType {
        let d = &self.s.trim().to_string();
        match d.len() {
            10 => {
                // mm/dd/yyyy
                if &d[2..3] == "/" && &d[5..6] == "/" 
                || &d[2..3] == "-" && &d[5..6] == "-" {
                    return CSVType::Date((*self.s).to_string());
                }
                // yyyy/dd/mm
                else if &d[4..5] == "/" && &d[7..8] == "/" 
                || &d[4..5] == "-" && &d[7..8] == "-" {
                    return CSVType::Date((*self.s).to_string());
                } else {
                    match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(CSVTypeError::Parse(e)),
                    };
                }
            },
            9 => {
                // m/dd/yyyy
                if &d[1..2] == "/" && &d[4..5] == "/" 
                || &d[1..2] == "-" && &d[4..5] == "-" {
                    return CSVType::Date((*self.s).to_string());
                }
                // mm/d/yyyy
                else if &d[2..3] == "/" && &d[4..5] == "/" 
                || &d[2..3] == "-" && &d[4..5] == "-" {
                    return CSVType::Date((*self.s).to_string());
                }
                // yyyy/dd/m
                else if &d[4..5] == "/" && &d[7..8] == "/" 
                || &d[4..5] == "-" && &d[7..8] == "-" {
                    return CSVType::Date((*self.s).to_string());
                }
                // yyyy/d/mm
                else if &d[4..5] == "/" && &d[6..7] == "/" 
                || &d[4..5] == "-" && &d[6..7] == "-" {
                    return CSVType::Date((*self.s).to_string());
                } else {
                    match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(CSVTypeError::Parse(e)),
                    };
                }
            },
            8 => {
                // m/d/yyyy
                if &d[1..2] == "/" && &d[3..4] == "/" 
                || &d[1..2] == "-" && &d[3..4] == "-" {
                    return CSVType::Date((*self.s).to_string());
                }
                // yyyy/d/m
                else if &d[4..5] == "/" && &d[6..7] == "/" 
                || &d[4..5] == "-" && &d[6..7] == "-" {
                    return CSVType::Date((*self.s).to_string());
                } 
                // mm/dd/yy
                if &d[2..3] == "/" && &d[5..6] == "/" 
                || &d[2..3] == "-" && &d[5..6] == "-" {
                    return CSVType::Date((*self.s).to_string());
                } else {
                    match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(CSVTypeError::Parse(e)),
                    };
                }
            },
            7 => {
                // m/dd/yy
                if &d[1..2] == "/" && &d[4..5] == "/" 
                || &d[1..2] == "-" && &d[4..5] == "-" {
                    return CSVType::Date((*self.s).to_string());
                }
                // mm/d/yy
                else if &d[2..3] == "/" && &d[4..5] == "/" 
                || &d[2..3] == "-" && &d[4..5] == "-" {
                    return CSVType::Date((*self.s).to_string());
                } else {
                    match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(CSVTypeError::Parse(e)),
                    };
                }
            },
            6 => {
                // m/d/yy
                if &d[1..2] == "/" && &d[3..4] == "/" 
                || &d[1..2] == "-" && &d[3..4] == "-" {
                    return CSVType::Date((*self.s).to_string());
                }
                // yy/d/m
                else if &d[2..3] == "/" && &d[4..5] == "/" 
                || &d[2..3] == "-" && &d[4..5] == "-" {
                    return CSVType::Date((*self.s).to_string());
                } else {
                    match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(CSVTypeError::Parse(e)),
                    };
                }
            },
            _ => match self.s.parse::<String>() {
                Ok(s) => return CSVType::String(s),
                Err(e) => return CSVType::Error(CSVTypeError::Parse(e)),
            },
        }
    }    
}

