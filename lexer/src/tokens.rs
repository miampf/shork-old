/// All different token types that can be found
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub enum TokenType{
    // single character stuff
    LeftParen, RightParen, LeftCurlyParen, RightCurlyParen,
    LeftSquareParen, RightSquareParen, Comma, Dot, Plus, Minus,
    Asterisk, Pipe, AndSym, Roof, NewLine,

    // one or more characters
    Exclamation, ExclamationEqual, Equal, EqualEqual, Greater,
    GreaterEqual, GreaterGreater, Lesser, LesserEqual, LesserLesser,
    Slash, Colon, ColonColon, Percent,

    // Type literals and type keywords
    Identifier, Integer, IntegerType, Float, FloatType, Char, CharType,
    String, StringType, Boolean, BooleanType, Regex, RegexType, T,

    // general keywords
    Reef, Get, From, As, Define, And, Or, For, While, Do, In, If,
    Else, Return, Structure, Implement, Private,

    Eof
}

/// A token in a shork program
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
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

    /// get the contents as an int
    pub fn content_int(&self) -> isize{
        let mut slice = [0u8; 8];
        for i in 0..8{
            slice[i] = self.raw[i]
        }
        isize::from_ne_bytes(slice)
    }

    /// get the contents as a float
    pub fn content_float(&self) -> f64{
        let mut slice = [0u8; 8];
        for i in 0..8{
            slice[i] = self.raw[i]
        }
        f64::from_ne_bytes(slice)
    }

    /// get the contents as a string
    pub fn content_string(&self) -> Result<String, std::string::FromUtf8Error>{
        String::from_utf8(self.raw.clone())
    }

    /// get the contents as a char
    pub fn content_char(&self) -> char{
        self.content_string().unwrap().chars().next().unwrap()
    }

    /// get the contents as a bool
    pub fn content_bool(&self) -> bool{
        self.raw[0] == 1
    }
}