#!allow(dead_code)]

use std;

pub struct Charset {
    chars: Vec<char>,
}

/// Charset struct for use with the atlas
impl Charset {
    pub fn new() -> Charset {
        Charset {
            chars: Vec::new()
        }
    }

    pub fn alpha() -> Charset {
        Charset {
            chars: vec![
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
                'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
                'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
                'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']
        }
    }

    pub fn number() -> Charset {
        Charset {
            chars: vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0']
        }
    }

    pub fn common_punc() -> Charset {
        Charset {
            chars: vec![' ', '!', '"', '#', '£', '$', '%', '&', '\'', '(', ')',
                        '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>',
                        '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}',
                        '~', '¬']
        }
    }

    /// Combine two charsets
    pub fn and(mut self, other: Charset) -> Charset {
        for c in other.chars {
            if self.chars.contains(&c) {
                continue;
            } else {
                self.chars.push(c)
            }
        }
        self
    }

    pub fn into_iter(self) -> std::vec::IntoIter<char> {
        self.chars.into_iter()
    }
}
