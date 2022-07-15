use std::i32;

#[derive(Debug)]
pub enum TokenType {
    Identifier,
    Number,
    MetaTag,
    Newline,
    Comma,
}

impl std::cmp::PartialEq for TokenType {
    fn eq(&self, other: &TokenType) -> bool {
        match self {
            TokenType::Identifier => match other {
                TokenType::Identifier => true,
                _ => false,
            },
            TokenType::Number => match other {
                TokenType::Number => true,
                _ => false,
            },
            TokenType::MetaTag => match other {
                TokenType::MetaTag => true,
                _ => false,
            },
            TokenType::Newline => match other {
                TokenType::Newline => true,
                _ => false,
            },
            TokenType::Comma => match other {
                TokenType::Comma => true,
                _ => false,
            },
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub clip: String,
    pub t_type: TokenType,
}

impl Token {
    pub fn new(clip: String, t_type: TokenType) -> Token {
        Token { clip, t_type }
    }
    pub fn tokenizer(s: &str) -> Vec<Token> {
        let mut counter = 0;
        let mut token_list: Vec<Token> = Vec::new();
        let mut state_code = 0;
        let s_collected: Vec<char> = s.to_uppercase().chars().collect();
        let mut buffer = String::new();
        while counter < s_collected.len() {
            match state_code {
                0 => match s_collected[counter] {
                    // Main loop
                    ' ' | '\t' => {}
                    '\n' => {
                        token_list.push(Token::new(String::from("\n"), TokenType::Newline));
                    }
                    '@' => {
                        // Meta tags
                        buffer.push('@');
                        state_code = 1;
                    }
                    ';' => {
                        // Comment
                        state_code = 2;
                    }
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        // Number literal
                        buffer.push(s_collected[counter]);
                        state_code = 3;
                    }
                    'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm'
                    | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y'
                    | 'z' | 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K'
                    | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W'
                    | 'X' | 'Y' | 'Z' | '_' | '$' | '.' => {
                        buffer.push(s_collected[counter]);
                        state_code = 4;
                    }
                    ',' => {
                        token_list.push(Token::new(String::from(","), TokenType::Comma));
                    }
                    '0' => {
                        // May be hexadecimal
                        state_code = 5;
                    }
                    _ => {}
                },
                1 => {
                    // Meta Tag
                    if s_collected[counter] == ' '
                        || s_collected[counter] == '\n'
                        || s_collected[counter] == '\t'
                    {
                        token_list.push(Token::new(buffer, TokenType::MetaTag));
                        buffer = String::new();
                        // Back to main loop;
                        state_code = 0;
                        continue;
                    } else {
                        buffer.push(s_collected[counter]);
                    }
                }
                2 => {
                    // Comment
                    if s_collected[counter] == '\n' {
                        token_list.push(Token::new(String::from("\n"), TokenType::Newline));
                        state_code = 0;
                    }
                }
                3 => {
                    // Number literal
                    if s_collected[counter] >= '0' && s_collected[counter] <= '9' {
                        // still a number
                        buffer.push(s_collected[counter]);
                    } else {
                        // not a number
                        token_list.push(Token::new(buffer, TokenType::Number));
                        buffer = String::new();
                        state_code = 0;
                        continue;
                    }
                }
                4 => {
                    // Identifier
                    if (s_collected[counter] >= 'A' && s_collected[counter] <= 'Z')
                        || (s_collected[counter] >= '0' && s_collected[counter] <= '9')
                        || (s_collected[counter] >= 'A' && s_collected[counter] <= 'Z')
                        || s_collected[counter] == '_'
                        || s_collected[counter] == '$'
                        || s_collected[counter] == '.'
                    {
                        // still an identifier,
                        buffer.push(s_collected[counter]);
                    } else {
                        token_list.push(Token::new(buffer, TokenType::Identifier));
                        buffer = String::new();
                        state_code = 0;
                        continue;
                    }
                }
                5 => {
                    // Look for '0x'
                    if buffer.len() == 0 && s_collected[counter] == 'X' {
                        state_code = 6;
                        counter += 1;
                        continue;
                    } else {
                        state_code = 0;
                        continue;
                    }
                }
                6 => {
                    // Hexadecimal Literal Parsing
                    if (s_collected[counter] >= '0' && s_collected[counter] <= '9')
                        || (s_collected[counter] >= 'A' && s_collected[counter] <= 'F')
                    {
                        buffer.push(s_collected[counter]);
                    } else {
                        // No longer a hexadecimal literal
                        let r =
                            i32::from_str_radix(&buffer, 16).expect("Cannot read hexdecimal value");
                        token_list.push(Token::new(r.to_string(), TokenType::Number));
                        buffer = String::new();
                        state_code = 0;
                        continue;
                    }
                }
                _ => {}
            }
            counter += 1;
        }
        token_list
    }
}
