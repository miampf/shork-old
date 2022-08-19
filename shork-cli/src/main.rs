use shork_error::{ShorkError, ErrorType, report::StderrReporter, report::Reporter};

fn main() {
    let e = ShorkError{
        e_type: ErrorType::SyntaxError,
        line: 2000,
        line_content: "test integer = 21".to_string(),
        pos_in_line: 5,
        message: "Expected ':'. Did you forget to add it after the variable name?".to_string()
    };
    let reporter = StderrReporter{};
    reporter.display_error(e);
}
