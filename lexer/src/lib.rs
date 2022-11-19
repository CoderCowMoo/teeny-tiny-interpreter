mod tokens;

use tokens::{Token, TokenType};
/// The main entry point for the interpreter. Refer to it's relevant documentation for more information.
struct Lexer {
    // we need to store the source code
    source: String,
    // and the current cur_pos in the source code
    cur_pos: i64,
    // and the current character
    cur_char: char,
}

impl Lexer {
    /// Returns a mutable Lexer instance.
    /// The instance calls next_char to initialise itself with the cur_pos of 0 and the first character of input.
    pub fn new(input_code: &String) -> Lexer {
        let mut lex = Self {
            source: input_code.clone(),
            cur_char: '\0' as char,
            cur_pos: -1,
        };
        lex.next_char();
        lex
    }

    /// Process the next char and update the cur_char field. Increments cur_pos by 1.
    pub fn next_char(&mut self) {
        self.cur_pos += 1;
        if self.cur_pos >= self.source.len().try_into().unwrap() {
            self.cur_char = '\0';
        } else {
            self.cur_char = self.source.chars().nth(self.cur_pos as usize).unwrap();
        }
    }

    /// Return the next char without consuming it.
    pub fn peek(&self) -> char {
        if self.cur_pos + 1 >= self.source.len() as i64 {
            return '\0';
        } else {
            return self.source.as_bytes()[self.cur_pos as usize + 1] as char;
        }
    }

    /// Return the lexer instance with no leading whitespace
    fn skip_whitespace(&mut self) {
        while self.cur_char == ' ' || self.cur_char == '\t' || self.cur_char == '\r' {
            self.next_char();
        }
    }

    /// skip comments
    fn skip_comment(&mut self) {
        if self.cur_char == '#' {
            while self.cur_char != '\n' {
                self.next_char();
            }
        }
    }

    /// Return the next token and consumes it. If the token is not a valid token, it panics.
    pub fn get_token(&mut self) -> Token {
        // get rid of whitespace
        self.skip_whitespace();
        // skip comments
        self.skip_comment();

        // data associated with token
        let mut s = String::new();
        s.push(self.cur_char);

        // here we goo
        let token_type = match self.cur_char {
            '+' => TokenType::PLUS,
            '-' => TokenType::MINUS,
            '*' => TokenType::ASTERISK,
            '/' => TokenType::SLASH,
            '=' => {
                if self.peek() == '=' {
                    self.next_char();
                    TokenType::EQEQ
                } else {
                    TokenType::EQ
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.next_char();
                    TokenType::GTEQ
                } else {
                    TokenType::GT
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.next_char();
                    TokenType::LTEQ
                } else {
                    TokenType::LT
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.next_char();
                    TokenType::NOTEQ
                } else {
                    panic!("Expected !=, got !{}", self.peek());
                }
            }
            // this lexes strings.
            '"' => {
                self.next_char();
                let start_pos = self.cur_pos;
                while self.cur_char != '"' {
                    match self.cur_char {
                        '\r' | '\n' | '\t' | '\\' | '%' => {
                            panic!("Illegal character in string.");
                        }
                        _ => {
                            self.next_char();
                        }
                    }
                }
                let tok_text = self
                    .source
                    .get(start_pos as usize..self.cur_pos as usize)
                    .unwrap();
                s = String::from(tok_text);
                TokenType::STRING
            }
            // now for numbers
            '0'..='9' => {
                let start_pos = self.cur_pos;
                while self.peek().is_ascii_digit() {
                    self.next_char();
                }
                // handle floats
                if self.peek() == '.' {
                    self.next_char();
                    if !self.peek().is_ascii_digit() {
                        panic!(
                            "Expected digit after decimal point. Instead, saw {}",
                            self.peek()
                        );
                    }
                    while self.peek().is_ascii_digit() {
                        self.next_char();
                    }
                }
                let tok_text = self
                    .source
                    .get(start_pos as usize..=self.cur_pos as usize)
                    .unwrap();
                s = String::from(tok_text);
                TokenType::NUMBER
            }
            // now for identifiers
            'a'..='z' | 'A'..='Z' => {
                let start_pos = self.cur_pos;
                while self.peek().is_ascii_alphanumeric() {
                    self.next_char();
                }
                let tok_text = self
                    .source
                    .get(start_pos as usize..(self.cur_pos as usize + 1))
                    .unwrap();
                let keyword = Token::check_if_keyword(tok_text);
                if keyword == TokenType::UNKNOWN {
                    // identifier
                    s = String::from(tok_text);
                    TokenType::IDENTIFIER
                } else {
                    s = String::from(tok_text);
                    keyword
                }
            }
            '\n' => TokenType::NEWLINE,
            '\0' => TokenType::EOF,
            _ => panic!("Unknown token: {}", self.cur_char),
        };
        // finished with getting token
        self.next_char();
        Token::new(token_type, s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_gen_works() {
        let mut lex = Lexer::new(&"".to_string());
        lex.next_char()
    }

    #[test]
    fn test_next_char() {
        let mut lex = Lexer::new(&"he".to_string());
        assert_eq!(lex.cur_char, 'h');
        lex.next_char();
        assert_eq!(lex.cur_char, 'e');
    }

    #[test]
    fn test_get_token() {
        let mut lex = Lexer::new(&"IF THEN ENDIF\nSTOP".to_string());
        let mut token: Token = Token {
            token_type: TokenType::UNKNOWN,
            value: "".to_string(),
        };
        while token != Token::new(TokenType::EOF, "\0".to_string()) {
            token = lex.get_token();
            println!("{:?}", token);
        }
    }

    #[test]
    fn test_string() {
        let mut lex = Lexer::new(&"\"Hello lads\"".to_string());
        assert_eq!(lex.get_token().value, "Hello lads");
    }

    #[test]
    fn test_numbers() {
        let mut lex = Lexer::new(&"123.456".to_string());
        assert_eq!(lex.get_token().value, "123.456");
    }

    #[test]
    fn test_double_op() {
        let mut lex = Lexer::new(&"==!=>=<==".to_string());
        assert_eq!(lex.get_token().token_type, TokenType::EQEQ);
        assert_eq!(lex.get_token().token_type, TokenType::NOTEQ);
        assert_eq!(lex.get_token().token_type, TokenType::GTEQ);
        assert_eq!(lex.get_token().token_type, TokenType::LTEQ);
        assert_eq!(lex.get_token().token_type, TokenType::EQ);
    }
}
