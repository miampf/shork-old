use shork_parser::tree::AST;
use shork_lexer::tokens::{Token, TokenType, TokenType::*};
use shork_error::{ShorkError, ErrorType};
use std::ops::{BitAnd, BitOr, Shl, Shr, Add, Sub, Div, Mul, Rem, Not, Neg};

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
        match self.r_type{
            0 => {
                if self.v_b.is_some() && other.get_boolean().is_some(){
                    return Some(self.v_b.unwrap().cmp(&other.get_boolean().unwrap()))
                }
                None
            },
            1 => {
                if self.v_i.is_some() && other.get_isize().is_some(){
                    return Some(self.v_i.unwrap().cmp(&other.get_isize().unwrap()))
                }
                None
            },
            2 => {
                if self.v_f.is_some() && other.get_float().is_some(){
                    return Some(self.v_f.unwrap().partial_cmp(&other.get_float().unwrap()).unwrap())
                }
                None
            },
            _ => {
                if self.v_s.is_some() && other.get_string().is_some(){
                    return Some(self.v_s.clone().unwrap().cmp(&other.get_string().clone().unwrap()))
                }
                None
            },
        }
    }
}

impl Neg for ShorkExprEvalResult{
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.r_type{
            1 => {
                if self.v_i.is_some(){
                    return Self::integer(-self.v_i.unwrap())
                }
                Self::error("None value in integer type. This is an error in the interpreter and not in your code".to_string())
            },
            2 => {
                if self.v_f.is_some(){
                    return Self::float(-self.v_f.unwrap())
                }
                Self::error("None value in float type. This is an error in the interpreter and not in your code".to_string())
            },
            _ => Self::error(format!("Unsupported Unary '-' on {}", self.get_type_string()))
        }
    }
}

impl Not for ShorkExprEvalResult{
    type Output = Self;

    fn not(self) -> Self::Output {
        match self.r_type{
            0 => {
                if self.v_b.is_some(){
                    return Self::boolean(!self.v_b.unwrap())
                }
                Self::error("None value in boolean type. This is an error in the interpreter and not in your code".to_string())
            },
            _ => Self::error(format!("Unsupported Unary '!' on {}", self.get_type_string()))
        }
    }
}

impl Rem for ShorkExprEvalResult{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match self.r_type{
            1 => {
                if self.v_i.is_some() && rhs.get_isize().is_some(){
                    return Self::integer(self.v_i.unwrap() % rhs.get_isize().unwrap())
                }
                Self::error("None value in integer type. This is an error in the interpreter and not in your code".to_string())
            },
            2 => {
                if self.v_f.is_some() && rhs.get_float().is_some(){
                    return Self::float(self.v_f.unwrap() % rhs.get_float().unwrap())
                }
                Self::error("None value in float type. This is an error in the interpreter and not in your code".to_string())
            },
            _ => Self::error(format!("Unsupported Operation '%' on {} and {}", self.get_type_string(), rhs.get_type_string()))
        }
    }
}

impl Mul for ShorkExprEvalResult{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self.r_type{
            1 => {
                if self.v_i.is_some() && rhs.get_isize().is_some(){
                    return Self::integer(self.v_i.unwrap() * rhs.get_isize().unwrap())
                }
                Self::error("None value in integer type. This is an error in the interpreter and not in your code".to_string())
            },
            2 => {
                if self.v_f.is_some() && rhs.get_float().is_some(){
                    return Self::float(self.v_f.unwrap() * rhs.get_float().unwrap())
                }
                Self::error("None value in float type. This is an error in the interpreter and not in your code".to_string())
            },
            _ => Self::error(format!("Unsupported Operation '*' on {} and {}", self.get_type_string(), rhs.get_type_string()))
        }
    }
}

impl Div for ShorkExprEvalResult{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match self.r_type{
            1 => {
                if self.v_i.is_some() && rhs.get_isize().is_some(){
                    return Self::integer(self.v_i.unwrap() / rhs.get_isize().unwrap())
                }
                Self::error("None value in integer type. This is an error in the interpreter and not in your code".to_string())
            },
            2 => {
                if self.v_f.is_some() && rhs.get_float().is_some(){
                    return Self::float(self.v_f.unwrap() / rhs.get_float().unwrap())
                }
                Self::error("None value in float type. This is an error in the interpreter and not in your code".to_string())
            },
            _ => Self::error(format!("Unsupported Operation '/' on {} and {}", self.get_type_string(), rhs.get_type_string()))
        }
    }
}

impl Sub for ShorkExprEvalResult{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self.r_type{
            1 => {
                if self.v_i.is_some() && rhs.get_isize().is_some(){
                    return Self::integer(self.v_i.unwrap() - rhs.get_isize().unwrap())
                }
                Self::error("None value in integer type. This is an error in the interpreter and not in your code".to_string())
            },
            2 => {
                if self.v_f.is_some() && rhs.get_float().is_some(){
                    return Self::float(self.v_f.unwrap() - rhs.get_float().unwrap())
                }
                Self::error("None value in float type. This is an error in the interpreter and not in your code".to_string())
            },
            _ => Self::error(format!("Unsupported Operation '+' on {} and {}", self.get_type_string(), rhs.get_type_string()))
        }
    }
}

impl Add for ShorkExprEvalResult{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self.r_type{
            1 => {
                if self.v_i.is_some() && rhs.get_isize().is_some(){
                    return Self::integer(self.v_i.unwrap() + rhs.get_isize().unwrap())
                }
                Self::error("None value in integer type. This is an error in the interpreter and not in your code".to_string())
            },
            2 => {
                if self.v_f.is_some() && rhs.get_float().is_some(){
                    return Self::float(self.v_f.unwrap() + rhs.get_float().unwrap())
                }
                Self::error("None value in float type. This is an error in the interpreter and not in your code".to_string())
            },
            3 => {
                if self.v_s.is_some() && rhs.get_string().is_some(){
                    return Self::string(self.v_s.unwrap() + rhs.get_string().clone().unwrap().as_str())
                }
                Self::error("None value in string type. This is an error in the interpreter and not in your code".to_string())
            },
            _ => Self::error(format!("Unsupported Operation '+' on {} and {}", self.get_type_string(), rhs.get_type_string()))
        }
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

        if self.match_t(r.val(), vec![EqualEqual, ExclamationEqual]){
            // we have an equality statement
            let childs = r.children();
            let left = self.equality(tree, childs[0])?;
            let right = self.equality(tree, childs[1])?;

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

        if res.is_err(){
            return res
        }

        if res.clone().unwrap().is_err(){
            // it's an error
            let e  = ShorkError::generate_error(ErrorType::InterpreterError, r.val().position(), self.source.clone(), res.unwrap().e_msg);
            res = Err(e)
        }

        res
    }

    /// evaluate bitwise operations
    fn bitwise(&mut self, tree: &mut AST, n: usize) -> Result<ShorkExprEvalResult, ShorkError>{
        let r = tree.get(n)?.clone();
        let mut res;

        if self.match_t(r.val(), vec![Pipe, And, LesserLesser, GreaterGreater]){
            // we have a bitwise statement
            let childs = r.children();
            let left = self.equality(tree, childs[0])?;
            let right = self.equality(tree, childs[1])?;

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

        if res.is_err(){
            return res
        }

        if res.clone().unwrap().is_err(){
            // it's an error
            let e  = ShorkError::generate_error(ErrorType::InterpreterError, r.val().position(), self.source.clone(), res.unwrap().e_msg);
            res = Err(e)
        }

        res
    }

    /// evaluate comparisons
    fn comparison(&mut self, tree: &mut AST, n: usize) -> Result<ShorkExprEvalResult, ShorkError>{
        let r = tree.get(n)?.clone();
        let mut res;

        if self.match_t(r.val(), vec![Greater, GreaterEqual, Lesser, LesserEqual]){
            // we have a bitwise statement
            let childs = r.children();
            let left = self.equality(tree, childs[0])?;
            let right = self.equality(tree, childs[1])?;

            res = match r.val().token_type(){
                Greater => Ok(ShorkExprEvalResult::boolean(left > right)),
                Lesser => Ok(ShorkExprEvalResult::boolean(left < right)),
                GreaterEqual => Ok(ShorkExprEvalResult::boolean(left >= right)),
                LesserEqual => Ok(ShorkExprEvalResult::boolean(left <= right)),
                _ => {
                    // This should NEVER happen
                    let e = ShorkError::generate_error(ErrorType::InterpreterError, r.val().position(), self.source.clone(), format!("Expected '>', '>=', '<' or '<=' found {:?}", r.val().token_type()));
                    Err(e)
                }
            }
        } else{
            res = self.term(tree, n);
        }

        if res.is_err(){
            return res
        }

        if res.clone().unwrap().is_err(){
            // it's an error
            let e  = ShorkError::generate_error(ErrorType::InterpreterError, r.val().position(), self.source.clone(), res.unwrap().e_msg);
            res = Err(e)
        }

        res
    }

    /// evaluate terms
    fn term(&mut self, tree: &mut AST, n: usize) -> Result<ShorkExprEvalResult, ShorkError>{
        let r = tree.get(n)?.clone();
        let mut res;

        let siblings;
        if r.parent().is_some(){
            siblings = tree.siblings(&r)?.len();
        } else {
            siblings = 0;
        }

        if self.match_t(r.val(), vec![Minus, Plus]) && (siblings == 2 || r.val().token_type() == &Plus){
            // we have a bitwise statement
            let childs = r.children();
            let left = self.equality(tree, childs[0])?;
            let right = self.equality(tree, childs[1])?;

            res = match r.val().token_type(){
                Minus => Ok(left - right),
                Plus => Ok(left + right),
                _ => {
                    // This should NEVER happen
                    let e = ShorkError::generate_error(ErrorType::InterpreterError, r.val().position(), self.source.clone(), format!("Expected '-' or '+' found {:?}", r.val().token_type()));
                    Err(e)
                }
            }
        } else{
            res = self.factor(tree, n);
        }

        if res.is_err(){
            return res
        }

        if res.clone().unwrap().is_err(){
            // it's an error
            let e  = ShorkError::generate_error(ErrorType::InterpreterError, r.val().position(), self.source.clone(), res.unwrap().e_msg);
            res = Err(e)
        }

        res
    }

    /// evaluate factors
    fn factor(&mut self, tree: &mut AST, n: usize) -> Result<ShorkExprEvalResult, ShorkError>{
        let r = tree.get(n)?.clone();
        let mut res;

        if self.match_t(r.val(), vec![Slash, Asterisk, Percent]){
            // we have a bitwise statement
            let childs = r.children();
            let left = self.equality(tree, childs[0])?;
            let right = self.equality(tree, childs[1])?;

            res = match r.val().token_type(){
                Slash => Ok(left / right),
                Asterisk => Ok(left * right),
                Percent => Ok(left % right),
                _ => {
                    // This should NEVER happen
                    let e = ShorkError::generate_error(ErrorType::InterpreterError, r.val().position(), self.source.clone(), format!("Expected '/', '*' or '%' found {:?}", r.val().token_type()));
                    Err(e)
                }
            }
        } else{
            res = self.unary(tree, n);
        }

        if res.is_err(){
            return res
        }

        if res.clone().unwrap().is_err(){
            // it's an error
            let e  = ShorkError::generate_error(ErrorType::InterpreterError, r.val().position(), self.source.clone(), res.unwrap().e_msg);
            res = Err(e)
        }

        res
    }

    /// evaluate unary expressions
    fn unary(&mut self, tree: &mut AST, n: usize) -> Result<ShorkExprEvalResult, ShorkError>{
        let r = tree.get(n)?.clone();
        let mut res;

        if self.match_t(r.val(), vec![Exclamation, Minus]){
            let right = self.equality(tree, n)?;
            res = match r.val().token_type(){
                Exclamation => Ok(!right),
                Minus => Ok(-right),
                _ => {
                    // This should NEVER happen
                    let e = ShorkError::generate_error(ErrorType::InterpreterError, r.val().position(), self.source.clone(), format!("Expected '!' or '-' found {:?}", r.val().token_type()));
                    Err(e)
                }
            }
        } else {
            res = self.primary(tree, n)
        }

        if res.is_err(){
            return res
        }

        if res.clone().unwrap().is_err(){
            // it's an error
            let e  = ShorkError::generate_error(ErrorType::InterpreterError, r.val().position(), self.source.clone(), res.unwrap().e_msg);
            res = Err(e)
        }

        res
    }

    /// evaluate unary expressions
    fn primary(&mut self, tree: &mut AST, n: usize) -> Result<ShorkExprEvalResult, ShorkError>{
        let r = tree.get(n)?.clone();
        let mut res = match r.val().token_type(){
            // the true values
            BooleanType => Ok(ShorkExprEvalResult::boolean(r.val().content_bool())),
            IntegerType => Ok(ShorkExprEvalResult::integer(r.val().content_int())),
            FloatType => Ok(ShorkExprEvalResult::float(r.val().content_float())),
            StringType => Ok(ShorkExprEvalResult::string(r.val().content_string().unwrap())),
            RegexType => Ok(ShorkExprEvalResult::string(r.val().content_string().unwrap())),
            _ => {
                let e = ShorkError::generate_error(ErrorType::InterpreterError, r.val().position(), self.source.clone(), format!("Unexpected symbol {:?}! Did you mean something else?", r.val().token_type()));
                Err(e)
            }
        };

        if res.is_err(){
            return res
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