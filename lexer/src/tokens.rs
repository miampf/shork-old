/// All different token types that can be found
#[derive(Debug, Clone)]
pub enum TokenType{
    // single character stuff
    LeftParen, RightParen, LeftCurlyParen, RightCurlyParen,
    LeftSquareParen, RightSquareParen, Comma, Dot, Plus, Minus,
    Slash, Asterisk, Pipe, AndSym, Roof, NewLine,

    // one or more characters
    Exclamation, ExclamationEqual, Equal, EqualEqual, Greater,
    GreaterEqual, GreaterGreater, Lesser, LesserEqual, LesserLesser,

    // Type literals
    Identifier, Integer, Long, Float, Double, Char, String, Boolean,
    Regex, T,

    // Keywords
    Reef, Get, From, As, Define, And, Or, For, While, Do, In, If,
    Else, Return, Structure, Implement, Private,

    Eof
}

/// A token in a shork program
#[derive(Debug, Clone)]
pub struct Token{
    t_type: TokenType,
    position: usize,
    length: usize,
    raw: String
}

impl Token{
    /// create a new token
    pub fn new(t_type: TokenType, position: usize, length: usize, raw: String) -> Self{
        Self { t_type, position, length, raw }
    }
}