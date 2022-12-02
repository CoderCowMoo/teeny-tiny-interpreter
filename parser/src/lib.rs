use cemitter::Emitter;
use lexer::tokens::{Token, TokenType};
use lexer::Lexer;
use std::collections::HashSet;

pub struct Parser<'a> {
    lexer: Lexer,
    emitter: &'a mut Emitter,
    cur_token: Token,
    peek_token: Token,
    // use a hashset (FOR SPEEEEEEEEEEEED NEEOOWWWWWW)
    symbols: HashSet<String>,
    labels_declared: HashSet<String>,
    labels_go_toed: HashSet<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer, emitter: &'a mut Emitter) -> Self {
        let mut newparser = Parser {
            lexer,
            emitter,
            cur_token: Token {
                token_type: TokenType::UNKNOWN,
                value: "".to_string(),
            },
            peek_token: Token {
                token_type: TokenType::UNKNOWN,
                value: "".to_string(),
            },
            symbols: HashSet::new(),
            labels_declared: HashSet::new(),
            labels_go_toed: HashSet::new(),
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

    // ----------------------------- IMPORTANT CHECKPOINT ----------------------------------
    // |     we will now begin implementing the function for each rule of the grammer      |
    // -------------------------------------------------------------------------------------
    /// Begin program. This is the inpoint for the user
    pub fn program(&mut self) {
        self.emitter.header_line("#include <stdio.h>".to_string());
        self.emitter.header_line("int main(void) {".to_string());

        // ignore excess newlines
        while self.check_token(TokenType::NEWLINE) {
            self.next_token();
        }
        // parse all the statements in this program
        while !self.check_token(TokenType::EOF) {
            self.statement();
        }

        // by this point program is done
        self.emitter.emit_line("return 0;".to_string());
        self.emitter.emit_line("}".to_string());

        // now check that each label in a GOTO exists
        for label in self.labels_go_toed.iter() {
            if !self.labels_declared.contains(label) {
                // its not in the declared labels list so die
                panic!("Attempting to GOTO an undeclared label: {}", label);
            }
        }
    }

    // Process each type of statement that we have defined. (e.g. PRINT | IF)
    fn statement(&mut self) {
        // ----- Here begins our gigantic if statements which we knew we eventually had to add eh?

        // is it a PRINT?
        // PRINT (expression | string)
        if self.check_token(TokenType::PRINT) {
            self.next_token();

            // check for string or expression.
            if self.check_token(TokenType::STRING) {
                self.emitter.emit_line("printf(\"{}\\n\");".to_string());
                self.next_token();
            } else {
                // then we have an expression to evaluate and print (e.g. 2+2)
                self.emitter
                    .emit("printf(\"%.2f\\n\", (float)(".to_string()); // printf("%.2f\n", (float)(EXPRESSION))
                self.expression(); // expression will print result
                self.emitter.emit_line("));".to_string()); // one bracket to close expression and 1 for printf
            }
        } else
        // IF statement?
        // IF comparison THEN nl { statement } ENDIF nl
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
        } else
        // WHILE comparison REPEAT { statement } ENDWHILE
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
        } else
        // a label for GOTO statements
        // LABEL ident
        if self.check_token(TokenType::LABEL) {
            println!("STATEMENT-LABEL");
            self.next_token();
            // make sure that the label doesn't already exist
            if self.labels_declared.contains(&self.cur_token.value) {
                panic!("Label already exists: {}", self.cur_token.value);
            }
            // it doesnt exist so add it now
            self.labels_declared
                .insert(self.cur_token.value.to_string());
            // make sure that there is a name for the LABEL
            self.match_token(TokenType::IDENTIFIER);
        } else
        // GOTO ident
        if self.check_token(TokenType::GOTO) {
            println!("STATEMENT-GOTO");
            self.next_token();
            // add the identifier to the gotoed HashSet
            self.labels_go_toed.insert(self.cur_token.value.to_string());
            self.match_token(TokenType::IDENTIFIER);
        } else
        // LET ident = expression
        if self.check_token(TokenType::LET) {
            println!("STATEMENT-LET");
            self.next_token();

            // check if it exists in symbol table, and declare if not
            if !self.symbols.contains(&self.cur_token.value) {
                self.symbols.insert(self.cur_token.value.to_string());
            }

            self.match_token(TokenType::IDENTIFIER);
            self.match_token(TokenType::EQ);

            self.expression();
        } else
        // INPUT ident
        if self.check_token(TokenType::INPUT) {
            println!("STATEMENT-INPUT");
            self.next_token();
            // if the variable doesn't exist already, declare it
            if !self.symbols.contains(&self.cur_token.value) {
                self.symbols.insert(self.cur_token.value.to_string());
            }
            // we gotta know what to input into
            self.match_token(TokenType::IDENTIFIER);
        } else {
            // invalid statement
            panic!(
                "Invalid statement at {} ({:?})",
                self.cur_token.value, self.cur_token.token_type
            );
        }
        // print a newline must be at end
        self.nl();
    }

    // evaluate a comparison
    // comparison ::= expression (("==" | "!=" | ">" | ">=" | "<" | "<=") expression)+
    // this means that a comparison is an expression with one or more other expressions
    // with a comparison operator between.
    fn comparison(&mut self) {
        println!("COMPARISON");
        self.expression();

        // must have at least one comp.op. and another expression afterwards.
        if self.is_comparison_operator() {
            self.next_token();
            self.expression();
        } else {
            panic!("Expected comparison operator at: {}", self.cur_token.value);
        }

        // we can now have 0 or more comp.op.s and expression pairs
        while self.is_comparison_operator() {
            self.next_token();
            self.expression();
        }
    }

    // helper to determine whether there is a comparison operator
    fn is_comparison_operator(&mut self) -> bool {
        self.check_token(TokenType::GT)
            || self.check_token(TokenType::GTEQ)
            || self.check_token(TokenType::LT)
            || self.check_token(TokenType::LTEQ)
            || self.check_token(TokenType::EQEQ)
            || self.check_token(TokenType::NOTEQ)
    }

    // evaluate an expression
    // expression ::= term {( "-" | "+" ) term}
    // an expression is a term optionally followed by a pos. or neg. term
    fn expression(&mut self) {
        println!("EXPRESSION");
        self.term();

        // can have 0 or more +/- expressions
        while self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
            self.next_token();
            self.term();
        }
    }

    // term ::= unary {( "/" | "*" ) unary}
    fn term(&mut self) {
        println!("TERM");
        self.unary();

        // we can have 0 or more * or / and expressions
        while self.check_token(TokenType::ASTERISK) || self.check_token(TokenType::SLASH) {
            self.next_token();
            self.unary();
        }
    }

    // unary ::= ["+" | "-"] primary
    fn unary(&mut self) {
        println!("UNARY");

        // optionally has a positive or negative
        if self.check_token(TokenType::PLUS) || self.check_token(TokenType::MINUS) {
            self.next_token();
        }
        self.primary();
    }

    // primary ::= number | ident
    fn primary(&mut self) {
        println!("PRIMARY ({})", self.cur_token.value);

        if self.check_token(TokenType::NUMBER) {
            self.next_token();
        } else if self.check_token(TokenType::IDENTIFIER) {
            // check that the variable exists before we allow it
            if !self.symbols.contains(&self.cur_token.value) {
                panic!(
                    "Referencing variable before assignment: {}",
                    self.cur_token.value
                );
            }
            self.next_token();
        } else {
            panic!("Unexpected token at {}", self.cur_token.value);
        }
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
mod tests {}
