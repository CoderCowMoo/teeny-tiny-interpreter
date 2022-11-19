#[derive(Clone, PartialEq, Debug)]
pub enum TokenType {
    UNKNOWN = -2,
    EOF,
    NEWLINE,
    NUMBER,     // make this accept floats and ints
    IDENTIFIER, // what is the name of the identifier
    STRING,
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

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        Self { token_type, value }
    }

    /// check if given string represents a keyword in the language or not
    pub fn check_if_keyword(token_text: &str) -> TokenType {
        match token_text {
            "LABEL" => TokenType::LABEL,
            "GOTO" => TokenType::GOTO,
            "PRINT" => TokenType::PRINT,
            "INPUT" => TokenType::INPUT,
            "LET" => TokenType::LET,
            "IF" => TokenType::IF,
            "THEN" => TokenType::THEN,
            "ENDIF" => TokenType::ENDIF,
            "WHILE" => TokenType::WHILE,
            "REPEAT" => TokenType::REPEAT,
            "ENDWHILE" => TokenType::ENDWHILE,
            _ => TokenType::UNKNOWN,
        }
    }
}
