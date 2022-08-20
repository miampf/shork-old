/// All different token types that can be found
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType{
    // single character stuff
    LeftParen, RightParen, LeftCurlyParen, RightCurlyParen,
    LeftSquareParen, RightSquareParen, Comma, Dot, Plus, Minus,
    Asterisk, Pipe, AndSym, Roof, NewLine,

    // one or more characters
    Exclamation, ExclamationEqual, Equal, EqualEqual, Greater,
    GreaterEqual, GreaterGreater, Lesser, LesserEqual, LesserLesser,
    Slash, Colon, ColonColon,

    // Type literals and type keywords
    Identifier, Integer, IntegerType, Float, FloatType, Char, CharType,
    String, StringType, Boolean, BooleanType, Regex, RegexType, T,

    // general keywords
    Reef, Get, From, As, Define, And, Or, For, While, Do, In, If,
    Else, Return, Structure, Implement, Private,

    Eof
}

/// A token in a shork program
#[derive(Debug, Clone, PartialEq)]
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

    /// get the token type
    pub fn token_type(&self) -> &TokenType{
        &self.t_type
    }

    /// get the tokens position
    pub fn position(&self) -> usize{
        self.position
    }

    /// get the tokens length
    pub fn len(&self) -> usize{
        self.length
    }

    /// get the raw contents
    pub fn raw(&self) -> &Vec<u8>{
        &self.raw
    }
}