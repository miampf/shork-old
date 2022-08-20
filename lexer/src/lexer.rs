use shork_error::report;
use crate::tokens;

/// The lexer that will generate tokens from a source string
pub struct Lexer<'a>{
    source: String,
    tokens: Vec<tokens::Token>,
    start: usize,
    current: usize,
    error_reporter: &'a mut dyn report::Reporter
}

impl<'a> Lexer<'a>{

    /// create a new Lexer with a source string
    pub fn new(source: String, error_reporter: &'a mut dyn report::Reporter) -> Self{
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            error_reporter
        }
    }

    /// scan for all tokens in the source and populate
    /// the tokens vector
    pub fn scan_tokens(&mut self) -> Result<(), shork_error::ShorkError>{
        // execute until we have consumed the whole source string
        while !(self.current >= self.source.len()) {
            self.start = self.current;

            let t = self.get_current_token()?;
            if t.is_some(){
                self.tokens.push(t.unwrap())
            }
        }

        self.tokens.push(tokens::Token::new(
            tokens::TokenType::Eof, 
            self.current+1, 
            0, Vec::new()));
        Ok(())
    }

    /// get the tokens (call after scan_tokens)
    pub fn get_tokens(&self) -> &Vec<tokens::Token>{
        &self.tokens
    }

    /// get the current token
    fn get_current_token(&mut self) -> Result<Option<tokens::Token>, shork_error::ShorkError>{
        use tokens::TokenType::*;

        let ch = self.advance()?;
        let t_type = match ch{
            '(' => Ok(Some(LeftParen)),
            ')' => Ok(Some(RightParen)),
            '{' => Ok(Some(LeftCurlyParen)),
            '}' => Ok(Some(RightCurlyParen)),
            '[' => Ok(Some(LeftSquareParen)),
            ']' => Ok(Some(RightSquareParen)),
            ',' => Ok(Some(Comma)),
            '.' => Ok(Some(Dot)),
            '+' => Ok(Some(Plus)),
            '-' => Ok(Some(Minus)),
            '*' => Ok(Some(Asterisk)),
            '|' => Ok(Some(Pipe)),
            '&' => Ok(Some(AndSym)),
            '^' => Ok(Some(Roof)),
            '!' => if self.does_match('=')? {self.current += 1; Ok(Some(ExclamationEqual))} else {Ok(Some(Exclamation))},
            '=' => if self.does_match('=')? {self.current += 1; Ok(Some(EqualEqual))} else {Ok(Some(Equal))},
            ':' => if self.does_match(':')? {self.current += 1; Ok(Some(ColonColon))} else {Ok(Some(Colon))},
            '>' => if self.does_match('=')? {self.current += 1; Ok(Some(GreaterEqual))} else {if self.does_match('>')? {self.current += 1;Ok(Some(GreaterGreater))} else {Ok(Some(Greater))}},
            '<' => if self.does_match('=')? {self.current += 1; Ok(Some(LesserEqual))} else {if self.does_match('<')? {self.current += 1;Ok(Some(LesserLesser))} else {Ok(Some(Lesser))}},
            '/' => {
                let mut ret = Ok(None);
                if self.does_match('/')?{
                    // it's a comment, consume it
                    while self.peek()? != '\n' && !(self.current >= self.source.len()) {
                        self.advance()?;
                    }
                } else {
                    ret = Ok(Some(Slash))
                }
                ret
            },
            '"' => {
                // this starts a string
                while self.peek()? != '"' && !(self.current >= self.source.len()){
                    self.advance()?;
                }
                if self.current >= self.source.len(){
                    let e = shork_error::ShorkError::generate_error(
                        shork_error::ErrorType::SyntaxError, 
                        self.start, 
                        self.source.clone(),
                        "Unterminated string detected".to_string());
                    self.error_reporter.add_error(e.clone());
                    return Err(e)
                }
                self.advance()?; // consume the closing "
                Ok(Some(StringType))
            },
            '\'' => {
                let inside = self.advance()?;
                if self.peek()? != '\'' && inside == '\''{
                    // This character contains too many symbols
                    let e = shork_error::ShorkError::generate_error(
                        shork_error::ErrorType::TypeError, 
                        self.current, 
                        self.source.clone(),
                        "To many symbols detected. A character type variable can only contain one symbol".to_string());
                    self.error_reporter.add_error(e.clone());
                    return Err(e)
                }
                Ok(Some(CharType))
            },
            '#' => {
                // this starts a string
                while self.peek()? != '#' && !(self.current >= self.source.len()){
                    self.advance()?;
                }
                if self.current >= self.source.len(){
                    let e = shork_error::ShorkError::generate_error(
                        shork_error::ErrorType::SyntaxError, 
                        self.start, 
                        self.source.clone(),
                        "Unterminated regex detected".to_string());
                    self.error_reporter.add_error(e.clone());
                    return Err(e)
                }
                self.advance()?; // consume the closing "
                Ok(Some(RegexType))
            },
            '\n' => {
                // since a new line can indicate the end of a statement,
                // we have to define when it is important
                let mut ret = Ok(None);

                let ch = self.peek_behind()?;
                let mut is_important = false;
                if ch.is_alphabetic() || ch.is_digit(10) || ch == ']' || ch == ')' || ch == '_' || ch == '#'{
                    is_important = true
                }

                if is_important{
                    ret = Ok(Some(NewLine))
                }
                
                ret
            },

            // skip over unused whitespace
            ' ' => Ok(None),
            '\r' => Ok(None),
            '\t' => Ok(None),

            _ => {
                let ret;
                // number parsing
                if ch.is_digit(10){
                    while self.peek()?.is_digit(10){self.advance()?;}

                    if self.peek()? == '.' && self.peek_two()?.is_digit(10){
                        // ladies, gentleman and enby friends, we got a float
                        self.advance()?; // consume the .

                        // consume the rest of the number
                        while self.peek()?.is_digit(10){self.advance()?;}
                        ret = Ok(Some(FloatType));
                    } else {
                        ret = Ok(Some(IntegerType))
                    }
                } else if ch.is_alphabetic() || ch == '_'{
                    // consume the whole identifier
                    while self.peek()?.is_alphanumeric(){self.advance()?;}

                    // check if it's a reserved keyword or not
                    let text = self.source[self.start..self.current].to_string();
                    let mut t_type = match text.as_str(){
                        "integer" => Some(Integer),
                        "float" => Some(Float),
                        "char" => Some(Char),
                        "string" => Some(String),
                        "boolean" => Some(Boolean),
                        "regex" => Some(Regex),
                        "T" => Some(T),
                        "reef" => Some(Reef),
                        "get" => Some(Get),
                        "from" => Some(From),
                        "as" => Some(As),
                        "define" => Some(Define),
                        "and" => Some(And),
                        "or" => Some(Or),
                        "for" => Some(For),
                        "while" => Some(While),
                        "do" => Some(Do),
                        "in" => Some(In),
                        "if" => Some(If),
                        "else" => Some(Else),
                        "return" => Some(Return),
                        "structure" => Some(Structure),
                        "implement" => Some(Implement),
                        "private" => Some(Private),

                        _ => None
                    };
                    if t_type.is_none(){
                        t_type = Some(Identifier)
                    }

                    ret = Ok(t_type)
                } else {
                    let e = shork_error::ShorkError::generate_error(
                        shork_error::ErrorType::LexicalError,
                        self.current,
                        self.source.clone(),
                        format!("Unexpected Symbol '{}'.", ch));
                    self.error_reporter.add_error(e.clone());
                    ret = Err(e);
                }

                ret
            }
        }?;

        if t_type.is_some(){
            return Ok(Some(tokens::Token::new(
            t_type.unwrap(), 
            self.start, 
            self.source[self.start..self.current].len(), 
            self.source[self.start..self.current].as_bytes().to_vec())))
        }
        return Ok(None)
    }

    /// peek at the current head
    fn peek(&mut self) -> Result<char, shork_error::ShorkError>{
        let ch = self.source.chars().nth(self.current);
        if ch.is_some(){
            return Ok(ch.unwrap())
        }

        // we failed to read
        let e = shork_error::ShorkError::generate_error(
            shork_error::ErrorType::ReadingError,
             self.current, 
             self.source.clone(), 
             "Couldn't read character at position".to_string());
        self.error_reporter.add_error(e.clone());
        Err(e)
    }

    /// peek at the current head + 1
    fn peek_two(&mut self) -> Result<char, shork_error::ShorkError>{
        let ch = self.source.chars().nth(self.current+1);
        if ch.is_some(){
            return Ok(ch.unwrap())
        }

        // we failed to read
        let e = shork_error::ShorkError::generate_error(
            shork_error::ErrorType::ReadingError,
             self.current+1, 
             self.source.clone(), 
             "Couldn't read character at position".to_string());
        self.error_reporter.add_error(e.clone());
        Err(e)
    }

    /// look one symbol behind you
    fn peek_behind(&mut self) -> Result<char, shork_error::ShorkError>{
        if self.current >= 2 {

            let ch = self.source.chars().nth(self.current-2);
            if ch.is_some(){
                return Ok(ch.unwrap())
            }
    
            // we failed to read
            let e = shork_error::ShorkError::generate_error(
                shork_error::ErrorType::ReadingError,
                 self.current-2, 
                 self.source.clone(), 
                 "Couldn't read character at position".to_string());
            self.error_reporter.add_error(e.clone());
            return Err(e)
        }
        Ok('\0')
    }

    /// check if a current character matches the prediction
    fn does_match(&mut self, ch_c: char) -> Result<bool, shork_error::ShorkError>{
        if self.current >= self.source.len() {return Ok(false)}; // EOF

        let ch = self.source.chars().nth(self.current);
        if ch.is_some(){
            return Ok(ch.unwrap() == ch_c)
        }

        // we failed to read
        let e = shork_error::ShorkError::generate_error(
            shork_error::ErrorType::ReadingError,
             self.current, 
             self.source.clone(), 
             "Couldn't read character at position".to_string());
        self.error_reporter.add_error(e.clone());
        Err(e)
    }

    /// advance the head and return the char at the
    /// new position
    fn advance(&mut self) -> Result<char, shork_error::ShorkError>{
        self.current += 1;
        let ch = self.source.chars().nth(self.current-1);
        if ch.is_some(){
            return Ok(ch.unwrap())
        }

        // we failed to read
        let e = shork_error::ShorkError::generate_error(
            shork_error::ErrorType::ReadingError,
             self.current-1, 
             self.source.clone(), 
             "Couldn't read character at position".to_string());
        self.error_reporter.add_error(e.clone());
        Err(e)
    }
}