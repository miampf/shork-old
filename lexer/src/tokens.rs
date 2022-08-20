/// All different token types that can be found
#[derive(Debug, Clone)]
pub enum TokenType{
    // single character stuff
    LeftParen, RightParen, LeftCurlyParen, RightCurlyParen,
    LeftSquareParen, RightSquareParen, Comma, Dot, Plus, Minus,
    Asterisk, Pipe, AndSym, Roof, NewLine,

    // one or more characters
    Exclamation, ExclamationEqual, Equal, EqualEqual, Greater,
    GreaterEqual, GreaterGreater, Lesser, LesserEqual, LesserLesser,
    Slash,

    // Type literals and type keywords
    Identifier, Integer, IntegerType, Float, FloatType, Char, CharType,
    String, StringType, Boolean, BooleanType, Regex, RegexType, T,

    // general keywords
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
    raw: Vec<u8>
}

impl Token{
    /// create a new token
    pub fn new(t_type: TokenType, position: usize, length: usize, raw: Vec<u8>) -> Self{
        Self { t_type, position, length, raw }
    }
}