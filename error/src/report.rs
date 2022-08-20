use std::io::{Write, stderr};
use crate::ShorkError;

/// A trait that must be implemented by all Error reporters
pub trait Reporter{
        /// Display the error you were given
    fn display_error(&self, e: ShorkError);

    /// Add an error to the queue
    fn add_error(&mut self, e: ShorkError);

    /// Get all errors in the queue
    fn get_errors(&self) -> &Vec<ShorkError>;
}

/// Report an error to the stderr channel
#[derive(Debug, Clone)]
pub struct StderrReporter{
    errors: Vec<ShorkError>
}

impl StderrReporter{
    /// create a new, empty reporter
    pub fn new() -> Self{
        Self { errors: Vec::new() }
    }
}

impl Reporter for StderrReporter{
    fn display_error(&self, e: ShorkError) {
        // pinpoint an arrow to the position in line
        let mut arrow = "".to_string();
        for _ in 0..e.pos_in_line {
            arrow += " "
        }

        // if there is a number that takes more space than a single
        // character (e.g. 20), adjust the number of spaces
        let line_len = e.line.to_string().len();
        for _ in 1..line_len {
            arrow += " "
        }

        arrow += "^----- Here";

        // construct the final message
        let string = format!(r#"
{} at line {}:
    {} | {}
       {}
{}
        "#, e.e_type, e.line, e.line, e.line_content, arrow, e.message);

        // if the error couldnt be printed to stderr, report
        // and print to stdout
        match stderr().write(string.as_bytes()){
            Ok(_) => {},
            Err(e) => {
                println!("While printing an error, another error occurred:\n{}", e);
                println!("The error above was caused by this error message:\n{}", string)
            }
        }
    }

    fn add_error(&mut self, e: ShorkError) {
        self.errors.push(e)
    }

    fn get_errors(&self) -> &Vec<ShorkError> {
        &self.errors
    }
}