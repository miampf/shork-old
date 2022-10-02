use shork_error::{report::Reporter, ShorkError, ErrorType};
use shork_lexer::tokens::{Token, TokenType::{*, self}};
use crate::tree::{AST, Node};

/// Build an AST for expressions given a token stream
pub struct ExprParser<'a>{
    current: usize,
    tokens: Vec<Token>,
    tree: AST,
    id_offset: usize,
    source: std::string::String,
    error_reporter: &'a mut dyn Reporter
}

impl<'a> ExprParser<'a>{
    /// create a new expression parser
    pub fn new(tokens: Vec<Token>, id_offset: usize, error_reporter: &'a mut dyn Reporter, source: std::string::String) -> Self{
        Self { current: 0, tokens, tree: AST::new(), id_offset, error_reporter, source }
    }

    /// Parse the given expression and populate the AST
    pub fn parse(&mut self) {
        self.tree = self.expression(); // start the recursive descent parser
    }

    /// parse expressions
    fn expression(&mut self) -> AST{
        self.equality()
    }

    /// parse equality expressions
    fn equality(&mut self) -> AST{
        // get the left side of the tree
        let mut tree = self.comparison();
        
        // is there a != or a ==?
        while self.match_token(vec![ExclamationEqual, EqualEqual]){
            self.id_offset += 1;

            let operator = self.previous();
            let mut n = Node::new(self.id_offset, operator, None, vec![]);
            self.comparison().clone_into_tree(&mut tree); // add the right side
            
            // make the operator the root node for everyone
            for c in tree.root(){
                n.add_child(c);
            }
            tree.set_root_all(n.id());
            tree.add(n.clone());
        }

        tree
    }
    
    /// parse comparisons
    fn comparison(&mut self) -> AST{
        // get the left side of the tree
        let mut tree = self.bitwise();
        
        // is there a >, >=, < or <=?
        while self.match_token(vec![Greater, GreaterEqual, Lesser, LesserEqual]){
            // check for unexpected tokens
            self.check_error(vec![LeftParen, Exclamation, Minus, IntegerType, FloatType], format!("Expected number, found {:?}", self.peek().token_type()));

            self.id_offset += 1;

            let operator = self.previous();
            let mut n = Node::new(self.id_offset, operator, None, vec![]);
            self.bitwise().clone_into_tree(&mut tree); // add the right side
            
            // make the operator the root node for everyone
            for c in tree.root(){
                n.add_child(c);
            }
            tree.set_root_all(n.id());
            tree.add(n.clone());
        }
        
        tree
    }
    
    /// parse bitwise expressions
    fn bitwise(&mut self) -> AST{
        // get the left side of the tree
        let mut tree = self.term();
        
        // is there a |, &, << or >>?
        while self.match_token(vec![Pipe, And, LesserLesser, GreaterGreater]){
            // check for unexpected tokens
            self.check_error(vec![LeftParen, Exclamation, Minus, IntegerType, FloatType], format!("Expected number, found {:?}", self.peek().token_type()));

            self.id_offset += 1;

            let operator = self.previous();
            let mut n = Node::new(self.id_offset, operator, None, vec![]);
            self.term().clone_into_tree(&mut tree); // add the right side
            
            // make the operator the root node for everyone
            for c in tree.root(){
                n.add_child(c);
            }
            tree.set_root_all(n.id());
            tree.add(n.clone());
        }

        tree
    }

    /// parse terms
    fn term(&mut self) -> AST{
        // get the left side of the tree
        let mut tree = self.factor();
        
        // is there a - or +?
        while self.match_token(vec![Minus, Plus]){
            // check for unexpected tokens
            self.check_error(vec![LeftParen, Exclamation, Minus, IntegerType, FloatType, StringType], format!("Expected number, found {:?}", self.peek().token_type()));

            self.id_offset += 1;

            let operator = self.previous();
            let mut n = Node::new(self.id_offset, operator, None, vec![]);
            self.factor().clone_into_tree(&mut tree); // add the right side
            
            // make the operator the root node for everyone
            for c in tree.root(){
                n.add_child(c);
            }
            tree.set_root_all(n.id());
            tree.add(n.clone());
        }

        tree
    }

    /// parse factors
    fn factor(&mut self) -> AST{
        // get the left side of the tree
        let mut tree = self.unary();
        
        // is there a /, * or %?
        while self.match_token(vec![Slash, Asterisk, Percent]){
            // check for unexpected tokens
            self.check_error(vec![LeftParen, Exclamation, Minus, IntegerType, FloatType], format!("Expected number, found {:?}", self.peek().token_type()));

            self.id_offset += 1;

            let operator = self.previous();
            let mut n = Node::new(self.id_offset, operator, None, vec![]);
            self.unary().clone_into_tree(&mut tree); // add the right side
            
            // make the operator the root node for everyone
            for c in tree.root(){
                n.add_child(c);
            }
            tree.set_root_all(n.id());
            tree.add(n.clone());
        }

        tree
    }

    /// parse unary operations
    fn unary(&mut self) -> AST{
        // is there a ! or -?
        if self.match_token(vec![Exclamation, Minus]){
            // handle unexpected tokens
            if self.previous().token_type() == &Exclamation{
                self.check_error(vec![BooleanType, Exclamation, LeftParen], format!("Expected a boolean, '!' or '(', found {:?}", self.peek().token_type()));
            }
            if self.previous().token_type() == &Minus{
                self.check_error(vec![IntegerType, FloatType, Minus], format!("Expected a number, found {:?}", self.peek().token_type()));
            }
            
            self.id_offset += 1;
            let operator = self.previous();
            let mut tree = self.unary();
            
            // make the operator the root node for everyone
            self.id_offset += 1;
            let n = Node::new(self.id_offset, operator, None, tree.root());
            tree.set_root_all(n.id());
            tree.add(n);
            
            println!("{:?}", tree);

            return tree;
        }

        self.primary()
    }

    /// parse primaries
    fn primary(&mut self) -> AST{
        let mut tree = AST::new();

        // is there any literal?
        if self.match_token(vec![StringType, CharType, IntegerType, FloatType, RegexType, BooleanType]){
            self.id_offset += 1;
            
            let n = Node::new(self.id_offset, self.previous(), None, vec![]);
            tree.add(n);
        }

        // is there a (?
        if self.match_token(vec![LeftParen]){
            self.expression().clone_into_tree(&mut tree); // group a new expression
            if !self.match_token(vec![RightParen]){
                // missing right parentheses
                let t = self.previous();
                let e = ShorkError::generate_error(ErrorType::ParserError, t.position(), self.source.clone(), "Expected ')'".to_string());
                self.error_reporter.add_error(e);
            }
        }

        tree
    }

    /// look if the current token matches one of the given token types
    /// and advance if so
    fn match_token(&mut self, t_types: Vec<TokenType>) -> bool{
        for t_type in t_types{
            if self.check(t_type){
                self.advance();
                return true;
            }
        }

        false
    }

    /// check if the current token is of a given type
    fn check(&self, t_type: TokenType) -> bool{
        if self.at_end() {return false;}
        self.peek().token_type() == &t_type
    }

    /// check if the parser reached the end of the token stream
    fn at_end(&self) -> bool{
        self.peek().token_type() == &Eof
    }

    /// get the current token
    fn peek(&self) -> Token{
        self.tokens[self.current].clone()
    }

    /// check if the token is of one of the given types without advancing
    fn match_token_no_advance(&self, t_types: Vec<TokenType>) -> bool{
        if self.at_end(){return false}
        for t_type in t_types{
            if self.tokens[self.current].token_type() == &t_type{
                return true;
            }
        }

        false
    }

    /// advance one position
    fn advance(&mut self) -> Token{
        if !self.at_end() {self.current += 1}
        self.previous()
    }

    /// return the previous token
    fn previous(&self) -> Token{
        self.tokens[self.current-1].clone()
    }

    /// check for an unexpected token.
    fn check_error(&mut self, expected_tokens: Vec<TokenType>, error_message: std::string::String) {
        if !self.match_token_no_advance(expected_tokens){
            let e = ShorkError::generate_error(ErrorType::ParserError, self.peek().position(), self.source.clone(), error_message);
            self.error_reporter.add_error(e);
        }
    }

    /// get the tree
    pub fn tree(&self) -> &AST{
        &self.tree
    }
}