use shork_lexer::{lexer::Lexer};
use shork_error::report::{StderrReporter, Reporter};
use shork_parser:: expressions::ExprParser;

fn main() {
    let program = r#"
reef examples
get std::io
define example(){
    me = 44.5
    if me == 22{
        io::print("me")
    }
    io::print("Oh hi there")
}
        "#.to_string();
    
    let mut reporter = StderrReporter::new();
    let mut l = Lexer::new(program, &mut reporter);
    l.scan_tokens().unwrap();
    
    // this expression doesn't make any sense and is just for testing
    let expr = "(2 + 2 == 2 * 2 | 1) + false << !!true - 6 * \"Hello\" / 'a'".to_string();
    
    let mut reporter = StderrReporter::new();
    let mut l = Lexer::new(expr.clone(), &mut reporter);
    l.scan_tokens().unwrap();
    for token in l.get_tokens(){
        println!("{:?}", token)
    }
    
    let mut p = ExprParser::new(l.get_tokens().clone(), 0, &mut reporter, expr);
    p.parse();

    let mut t = p.tree().clone();
    t.print();


    for e in reporter.get_errors(){
        reporter.display_error(e.clone())
    }
}
