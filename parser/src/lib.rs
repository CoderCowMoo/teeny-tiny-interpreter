use lexer::tokens::{Token, TokenType};
use lexer::Lexer;

pub struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut newparser = Parser {
            lexer,
            cur_token: Token {
                token_type: TokenType::UNKNOWN,
                value: "".to_string(),
            },
            peek_token: Token {
                token_type: TokenType::UNKNOWN,
                value: "".to_string(),
            },
        };
        // initialise cur_token and peek_token.
        // due to functions, it initialises peek_token first and then cur_token
        // calling twice will give cur_token the first token and peek_token the second.
        newparser.next_token();
        newparser.next_token();
        newparser
    }

    pub fn check_token(&self, kind: TokenType) -> bool {
        kind == self.cur_token.token_type
    }

    pub fn check_peek(&self, kind: TokenType) -> bool {
        kind == self.cur_token.token_type
    }

    /// Try and match the current token to the next expected token.
    pub fn match_token(&mut self, kind: TokenType) {
        if !self.check_token(kind.clone()) {
            panic!("Expected token {:?}, got {:?}", kind, self.cur_token);
        }
        self.next_token();
    }

    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.get_token();
    }

    // we will now begin implementing the function for each rule of the grammer
    /// Begin program. This is the inpoint for the user
    pub fn program(&mut self) {
        println!("PROGRAM");
        while !self.check_token(TokenType::EOF) {
            self.statement();
        }
    }

    // Process each type of statement that we have defined. (e.g. PRINT | IF)
    fn statement(&mut self) {
        // is it a PRINT?
        if self.check_token(TokenType::PRINT) {
            println!("STATEMENT-PRINT");
            self.next_token();

            // check for string or expression.
            if self.check_token(TokenType::STRING) {
                self.next_token();
            } else {
                // then we have an expression to evaluate and print (e.g. 2+2)
                todo!();
            }
        }
        // IF statement?
        if self.check_token(TokenType::IF) {
            println!("STATEMENT-IF");
            self.next_token();
            self.comparison();

            // needs to have a THEN after comparison expression.
            self.match_token(TokenType::THEN);
            self.nl();

            // in the body of the IF, we'll have zero or more statements
            while !self.check_token(TokenType::ENDIF) {
                self.statement();
            }

            // we need to have an ENDIF eventually after IF
            self.match_token(TokenType::ENDIF);
        }
        // WHILE?
        if self.check_token(TokenType::WHILE) {
            println!("STATEMENT-WHILE");
            self.next_token();
            self.comparison();

            // like for IF we had THEN, WHILE has REPEAT
            self.match_token(TokenType::REPEAT);
            self.nl();

            // then the statements in the body
            while !self.check_token(TokenType::ENDWHILE) {
                self.statement();
            }
            // even though we check for it previously, _make sure_ that it is there.
            self.match_token(TokenType::ENDWHILE);
        }
        // print a newline must be at end
        self.nl();
    }

    fn comparison(&mut self) {
        todo!();
    }

    // a newline
    fn nl(&mut self) {
        println!("NEWLINE");

        // we need at least one newline
        self.match_token(TokenType::NEWLINE);
        // but allow for more
        while self.check_token(TokenType::NEWLINE) {
            self.next_token();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
