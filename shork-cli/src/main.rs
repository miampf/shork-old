use shork_error::{ShorkError, ErrorType, report::StderrReporter, report::Reporter};

fn main() {
    let e = ShorkError::generate_error(ErrorType::SyntaxError, 52, r#"
reef examples
get std::io
define example(){
    io:print("Oh hi there")
}
    "#.to_string(), "Expected '.', found ':' instead.".to_string());
    let reporter = StderrReporter::new();
    reporter.display_error(e);
}
