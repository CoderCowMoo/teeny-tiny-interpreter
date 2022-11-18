mod tokens {
    pub enum TokenType {
        EOF,
        NEWLINE,
        NUMBER(usize),
        IDENTIFIER,
        STRING(String),
        // our lang keywords
        LABEL,
        GOTO,
        PRINT,
        INPUT,
        LET,
        IF,
        THEN,
        ENDIF,
        WHILE,
        REPEAT,
        ENDWHILE,
        // now for the operators
        EQ,
        PLUS,
        MINUS,
        ASTERISK,
        SLASH,
        EQEQ,
        NOTEQ,
        LT,
        LTEQ,
        GT,
        GTEQ,
    }
}
