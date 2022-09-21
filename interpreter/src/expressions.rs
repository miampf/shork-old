use shork_parser::tree::AST;
use shork_lexer::tokens::{Token, TokenType, TokenType::*};
use shork_error::{ShorkError, ErrorType};
use std::ops::{BitAnd, BitOr, Shl, Shr};

/// This contains an evaluated result
#[derive(Debug, Clone, PartialEq)]
pub struct ShorkExprEvalResult{
    r_type: u8, // 0=boolean,1=integer,2=float,3=string or char,4=regex(uses v_s),5=error
    v_b: Option<bool>,
    v_i: Option<isize>,
    v_f: Option<f64>,
    v_s: Option<std::string::String>,
    e_msg: std::string::String
}

/// The expression evaluator
#[derive(Debug, Clone)]
pub struct ExprEvaluator{
    source: std::string::String
}

// this next section contains operator implementations for ShorkExprEvalResult

impl PartialOrd for ShorkExprEvalResult{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let res = match self.r_type{
            0 => Some(self.v_b.cmp(&other.get_boolean())),
            1 => Some(self.v_i.cmp(&other.get_isize())),
            2 => Some(self.v_f.partial_cmp(&other.get_float()).unwrap()),
            _ => Some(self.v_s.cmp(other.get_string()))
        };

        res
    }
}

impl BitAnd for ShorkExprEvalResult{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let res;

        match self.r_type{
            0 => {
                res = match rhs.get_type(){
                    0 => Self::boolean(self.v_b.unwrap() & rhs.get_boolean().unwrap()),
                    _ => Self::error(format!("Unsupported Operation '&' on boolean and {}", rhs.get_type_string()))
                }
            },
            1 => {
                res = match rhs.get_type(){
                    1 => Self::integer(self.v_i.unwrap() & rhs.get_isize().unwrap()),
                    _ => Self::error(format!("Unsupported Operation '&' on integer and {}", rhs.get_type_string()))
                }
            },
            _ => {
                res = Self::error(format!("Unsupported Operation '&' on {} and {}", self.get_type_string(), rhs.get_type_string()))
            }
        }

        res
    }
}

impl BitOr for ShorkExprEvalResult{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let res;

        match self.r_type{
            0 => {
                res = match rhs.get_type(){
                    0 => Self::boolean(self.v_b.unwrap() | rhs.get_boolean().unwrap()),
                    _ => Self::error(format!("Unsupported Operation '|' on boolean and {}", rhs.get_type_string()))
                }
            },
            1 => {
                res = match rhs.get_type(){
                    1 => Self::integer(self.v_i.unwrap() | rhs.get_isize().unwrap()),
                    _ => Self::error(format!("Unsupported Operation '|' on integer and {}", rhs.get_type_string()))
                }
            },
            _ => {
                res = Self::error(format!("Unsupported Operation '|' on {} and {}", self.get_type_string(), rhs.get_type_string()))
            }
        }

        res
    }
}

impl Shl for ShorkExprEvalResult{
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        let res;

        match self.r_type{
            1 => {
                res = match rhs.get_type(){
                    1 => Self::integer(self.v_i.unwrap() << rhs.get_isize().unwrap()),
                    _ => Self::error(format!("Unsupported Operation '<<' on integer and {}", rhs.get_type_string()))
                }
            },
            _ => {
                res = Self::error(format!("Unsupported Operation '|' on {} and {}", self.get_type_string(), rhs.get_type_string()))
            }
        }

        res
    }
}

impl Shr for ShorkExprEvalResult{
    type Output = Self;

    fn shr(self, rhs: Self) -> Self::Output {
        let res;

        match self.r_type{
            1 => {
                res = match rhs.get_type(){
                    1 => Self::integer(self.v_i.unwrap() >> rhs.get_isize().unwrap()),
                    _ => Self::error(format!("Unsupported Operation '<<' on integer and {}", rhs.get_type_string()))
                }
            },
            _ => {
                res = Self::error(format!("Unsupported Operation '|' on {} and {}", self.get_type_string(), rhs.get_type_string()))
            }
        }

        res
    }
}

impl ShorkExprEvalResult{
    /// create a boolean result
    pub fn boolean(val: bool) -> Self{
        Self { r_type: 0, v_b: Some(val), v_i: None, v_f: None, v_s: None, e_msg: "".to_string() }
    }

    /// create a isize result
    pub fn integer(val: isize) -> Self{
        Self { r_type: 1, v_b: None, v_i: Some(val), v_f: None, v_s: None, e_msg: "".to_string() }
    }

    /// create a float result
    pub fn float(val: f64) -> Self{
        Self { r_type: 2, v_b: None, v_i: None, v_f: Some(val), v_s: None, e_msg: "".to_string() }
    }

    /// create a string result
    pub fn string(val: std::string::String) -> Self{
        Self { r_type: 3, v_b: None, v_i: None, v_f: None, v_s: Some(val), e_msg: "".to_string() }
    }

    /// create a regex result
    pub fn regex(val: std::string::String) -> Self{
        Self { r_type: 4, v_b: None, v_i: None, v_f: None, v_s: Some(val), e_msg: "".to_string() }
    }

    /// create an error (useful for operator implementations)
    pub fn error(msg: std::string::String) -> Self{
        Self { r_type: 5, v_b: None, v_i: None, v_f: None, v_s: None, e_msg: msg }
    }

    /// get the value as a boolean
    pub fn get_boolean(&self) -> Option<bool>{
        self.v_b
    }

    /// get the value as an integer
    pub fn get_isize(&self) -> Option<isize>{
        self.v_i
    }   

    /// get the value as a float
    pub fn get_float(&self) -> Option<f64>{
        self.v_f
    }

    /// get the value as a string
    pub fn get_string(&self) -> &Option<std::string::String>{
        &self.v_s
    }

    /// get the type of the result
    pub fn get_type(&self) -> u8{
        self.r_type
    }

    /// get the type as a string
    pub fn get_type_string(&self) -> std::string::String{
        match self.r_type{
            0 => return "boolean".to_string(),
            1 => return "integer".to_string(),
            2 => return "float".to_string(),
            3 => return "string".to_string(),
            4 => return "regex".to_string(),
            _ => return "error".to_string()
        }
    }

    /// check if its an error
    pub fn is_err(&self) -> bool{
        self.r_type == 5
    }
}

impl ExprEvaluator{
    pub fn new(source: std::string::String) -> Self{
        Self { source }
    }

    /// evaluate a expression tree
    pub fn evaluate(&mut self, tree: &mut AST) -> Result<ShorkExprEvalResult, ShorkError>{
        self.equality(tree, tree.root()[0])
    }

    /// evaluate equality expressions
    fn equality(&mut self, tree: &mut AST, n: usize) -> Result<ShorkExprEvalResult, ShorkError>{
        let r = tree.get(n)?.clone();
        let mut res;

        if !self.match_t(r.val(), vec![EqualEqual, ExclamationEqual]){
            // we have an equality statement
            let childs = r.children();
            let left = self.bitwise(tree, childs[0])?;
            let right = self.bitwise(tree, childs[1])?;

            res = match r.val().token_type(){
                ExclamationEqual => Ok(ShorkExprEvalResult::boolean(left != right)),
                EqualEqual => Ok(ShorkExprEvalResult::boolean(left == right)),
                _ => {
                    // This should NEVER happen
                    let e = ShorkError::generate_error(ErrorType::InterpreterError, r.val().position(), self.source.clone(), format!("Expected '==' or '!=', found {:?}", r.val().token_type()));
                    Err(e)
                }
            }
        } else{
            res = self.bitwise(tree, n);
        }
        if res.clone().unwrap().is_err(){
            // it's an error
            let e  = ShorkError::generate_error(ErrorType::InterpreterError, r.val().position(), self.source.clone(), res.unwrap().e_msg);
            res = Err(e)
        }

        res
    }

    /// evaluate bitwise operations
    pub fn bitwise(&mut self, tree: &mut AST, n: usize) -> Result<ShorkExprEvalResult, ShorkError>{
        let r = tree.get(n)?.clone();
        let mut res;

        if !self.match_t(r.val(), vec![Pipe, And, LesserLesser, GreaterGreater]){
            // we have a bitwise statement
            let childs = r.children();
            let left = self.comparison(tree, childs[0])?;
            let right = self.comparison(tree, childs[1])?;

            res = match r.val().token_type(){
                Pipe => Ok(left | right),
                And => Ok(left & right),
                LesserLesser => Ok(left << right),
                GreaterGreater => Ok(left >> right),
                _ => {
                    // This should NEVER happen
                    let e = ShorkError::generate_error(ErrorType::InterpreterError, r.val().position(), self.source.clone(), format!("Expected '|', '&', '<<' or '>>' found {:?}", r.val().token_type()));
                    Err(e)
                }
            }
        } else{
            res = self.comparison(tree, n);
        }
        if res.clone().unwrap().is_err(){
            // it's an error
            let e  = ShorkError::generate_error(ErrorType::InterpreterError, r.val().position(), self.source.clone(), res.unwrap().e_msg);
            res = Err(e)
        }

        res
    }

    /// evaluate comparisons
    pub fn comparison(&mut self, tree: &mut AST, n: usize) -> Result<ShorkExprEvalResult, ShorkError>{
        let r = tree.get(n)?.clone();
        let mut res;

        if !self.match_t(r.val(), vec![Greater, GreaterEqual, Lesser, LesserEqual]){
            // we have a bitwise statement
            let childs = r.children();
            let left = self.term(tree, childs[0])?;
            let right = self.term(tree, childs[1])?;

            res = match r.val().token_type(){
                Greater => Ok(ShorkExprEvalResult::boolean(left > right)),
                Lesser => Ok(ShorkExprEvalResult::boolean(left < right)),
                GreaterEqual => Ok(ShorkExprEvalResult::boolean(left >= right)),
                LesserEqual => Ok(ShorkExprEvalResult::boolean(left <= right)),
                _ => {
                    // This should NEVER happen
                    let e = ShorkError::generate_error(ErrorType::InterpreterError, r.val().position(), self.source.clone(), format!("Expected '|', '&', '<<' or '>>' found {:?}", r.val().token_type()));
                    Err(e)
                }
            }
        } else{
            res = self.term(tree, n);
        }
        if res.clone().unwrap().is_err(){
            // it's an error
            let e  = ShorkError::generate_error(ErrorType::InterpreterError, r.val().position(), self.source.clone(), res.unwrap().e_msg);
            res = Err(e)
        }

        res
    }

    /// evaluate terms
    pub fn term(&mut self, tree: &mut AST, n: usize) -> Result<ShorkExprEvalResult, ShorkError>{
        let r = tree.get(n)?.clone();
        let mut res;

        if !self.match_t(r.val(), vec![Greater, GreaterEqual, Lesser, LesserEqual]){
            // we have a bitwise statement
            let childs = r.children();
            let left = self.term(tree, childs[0])?;
            let right = self.term(tree, childs[1])?;

            res = match r.val().token_type(){
                Greater => Ok(ShorkExprEvalResult::boolean(left > right)),
                Lesser => Ok(ShorkExprEvalResult::boolean(left < right)),
                GreaterEqual => Ok(ShorkExprEvalResult::boolean(left >= right)),
                LesserEqual => Ok(ShorkExprEvalResult::boolean(left <= right)),
                _ => {
                    // This should NEVER happen
                    let e = ShorkError::generate_error(ErrorType::InterpreterError, r.val().position(), self.source.clone(), format!("Expected '|', '&', '<<' or '>>' found {:?}", r.val().token_type()));
                    Err(e)
                }
            }
        } else{
            res = self.term(tree, n);
        }
        if res.clone().unwrap().is_err(){
            // it's an error
            let e  = ShorkError::generate_error(ErrorType::InterpreterError, r.val().position(), self.source.clone(), res.unwrap().e_msg);
            res = Err(e)
        }

        res
    }

    /// check if a token type is one of the given
    fn match_t(&self, token: &Token, t_types: Vec<TokenType>) -> bool{
        for t_type in t_types{
            if token.token_type() == &t_type{
                return true
            }
        }

        false
    }
}