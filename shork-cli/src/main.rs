use shork_lexer::{lexer::Lexer};
use shork_error::report::{StderrReporter, Reporter};
use shork_parser::expressions::ExprParser;
use shork_interpreter::expressions::ExprEvaluator;
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), std::io::Error>{
    loop{
        let mut buffer = String::new();
        print!("shork$ ");
        stdout().flush()?;
        stdin().read_line(&mut buffer)?;

        if buffer.to_lowercase().trim() == "exit"{
            return Ok(());
        }

        let mut reporter = StderrReporter::new();
        let mut l = Lexer::new(buffer.clone(), &mut reporter);
        l.scan_tokens().unwrap();
    
        let mut p = ExprParser::new(l.get_tokens().clone(), 0, &mut reporter, buffer.clone());
        p.parse();
        let mut tree = p.tree().clone();
    
        for e in reporter.clone().get_errors(){
            reporter.display_error(e.clone())
        }
    
        let mut interpreter = ExprEvaluator::new(buffer);
        let res = interpreter.evaluate(&mut tree);
    
        if res.is_err(){
            reporter.display_error(res.clone().unwrap_err());
            println!();
        } else {
            println!("{}", res.unwrap())
        }
    
    }
}
