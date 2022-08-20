pub mod report;

use std::{fmt::Display, error::Error};

/// All errors go back to this struct
#[derive(Clone, Debug)]
pub struct ShorkError{
    pub e_type: ErrorType,
    pub line: usize,
    pub line_content: String,
    pub pos_in_line: usize,
    pub message: String
}

/// Different error types that can be thrown with a ShorkError
#[derive(Copy, Clone, Debug)]
pub enum ErrorType{
    ReadingError,
    SyntaxError
}

impl Error for ShorkError{}

impl Display for ShorkError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // pinpoint an arrow to the position in line
        let mut arrow = "".to_string();
        for _ in 0..self.pos_in_line {
            arrow += " "
        }

        let line_len = self.line.to_string().len();
        for _ in 1..line_len {
            arrow += " "
        }

        arrow += "^----- Here";

        write!(f, r#"
{} at line {}:
    {} | {}
       {}
{}
        "#, self.e_type, self.line, self.line, self.line_content, arrow, self.message)
    }
}

impl Display for ErrorType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_print = match *self {
            Self::ReadingError => "Reading Error",
            Self::SyntaxError => "Syntax Error"
        };
        write!(f, "{}", to_print)
    }
}