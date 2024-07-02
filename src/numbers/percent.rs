use crate::types::ByteString;

impl ByteString {
    pub fn is_percent_pos(&self) -> bool {
        match self.bytestring.trim() {
            string if string.contains("%")
            => match string.chars().map(|x| x).collect::<Vec<char>>() {
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
        match self.bytestring.trim() {
            string if string.contains("%")
            && string.contains("-")
            => match string.chars().map(|x| x).collect::<Vec<char>>() {
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
}