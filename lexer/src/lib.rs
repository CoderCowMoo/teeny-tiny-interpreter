mod tokens;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_gen_works() {
        let mut lex = Lexer::new(&"".to_string());
    }

    #[test]
    fn test_next_char() {
        let mut lex = Lexer::new(&"he".to_string());
        assert_eq!(lex.cur_char, 'h');
        lex.next_char();
        assert_eq!(lex.cur_char, 'e');
    }
}
