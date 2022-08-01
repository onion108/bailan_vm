use std::i32;

macro_rules! token {
    ($buffer: expr, $tokenType: ident) => {
        Token::new($buffer.to_string(), TokenType::$tokenType)
    };
}

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
                        token_list.push(token!("\n", Newline));
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
                    '1'..='9' => {
                        // Number literal
                        buffer.push(s_collected[counter]);
                        state_code = 3;
                    }
                    'a'..='z' |'A'..='Z' | '_' | '$' | '.' => {
                        buffer.push(s_collected[counter]);
                        state_code = 4;
                    }
                    ',' => {
                        token_list.push(token!(String::from(","), Comma));
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
                        token_list.push(token!(buffer, MetaTag));
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
                        token_list.push(token!(String::from("\n"), Newline));
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
                        token_list.push(token!(buffer, Number));
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
                        token_list.push(token!(buffer, Identifier));
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
                        // Maybe just zero?
                        token_list.push(token!("0", Number));
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
                        token_list.push(token!(r, Number));
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
