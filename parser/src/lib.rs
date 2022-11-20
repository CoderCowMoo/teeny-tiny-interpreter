use lexer::tokens::{Token, TokenType};
use lexer::Lexer;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser {
            lexer,
            cur_token: Token {
                token_type: TokenType::UNKNOWN,
                value: "".to_string(),
            },
            peek_token: Token {
                token_type: TokenType::UNKNOWN,
                value: "".to_string(),
            },
        }
    }

    pub fn check_token(&self, kind: TokenType) -> bool {
        kind == self.cur_token.token_type
    }

    pub fn check_peek(&self, kind: TokenType) -> bool {
        kind == self.cur_token.token_type
    }

    /// Try and match the current token to the next expected token.
    pub fn match_token(&mut self, kind: TokenType) {
        if !self.check_token(kind) {
            panic!("Expected token {:?}, got {:?}", kind, self.cur_token);
        }
        self.next_token();
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token;
        self.peek_token = self.lexer.get_token();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
