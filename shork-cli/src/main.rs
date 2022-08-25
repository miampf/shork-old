use shork_lexer::lexer::Lexer;
use shork_error::report::StderrReporter;

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
    for token in l.get_tokens(){
        println!("{:?}", token)
    }
}
