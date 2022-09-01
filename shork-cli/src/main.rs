use shork_lexer::{lexer::Lexer, tokens::Token, tokens::TokenType};
use shork_error::report::StderrReporter;
use shork_parser::tree::{AST, Node};

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

    let mut ast = AST::new();

    let nodes = [
            Node::new(0, Token::new(TokenType::Plus, 1, 1, "+".to_string().into_bytes()), None, vec![1, 2]),
            Node::new(1, Token::new(TokenType::IntegerType, 0, 2, vec![23]), Some(0), vec![]),
            Node::new(2, Token::new(TokenType::IntegerType, 2, 1, vec![2]), Some(0), vec![])
        ];
            
    for n in &nodes{
        ast.add(n)
    }

    ast.print()

}
