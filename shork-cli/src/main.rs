use shork_lexer::{lexer::Lexer};
use shork_error::report::{StderrReporter, Reporter};
use shork_parser::expressions::ExprParser;
use shork_interpreter::expressions::ExprEvaluator;

fn main() {
    /*
    * Current error cases:
    *   (3*2)%2
    *   !true // has stack overflow?!?!
    *   -2
    */
    let expr = "2-2".to_string();
    let mut reporter = StderrReporter::new();
    let mut l = Lexer::new(expr.clone(), &mut reporter);
    l.scan_tokens().unwrap();

    let mut p = ExprParser::new(l.get_tokens().clone(), 0, &mut reporter, expr.clone());
    p.parse();
    let mut tree = p.tree().clone();

    for e in reporter.clone().get_errors(){
        reporter.display_error(e.clone())
    }

    let mut interpreter = ExprEvaluator::new(expr);
    let res = interpreter.evaluate(&mut tree);

    if res.is_err(){
        reporter.display_error(res.unwrap_err());
        return;
    }

    println!("{:?}", res.unwrap())
}
