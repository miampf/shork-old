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
            Node::new(0, Token::new(TokenType::And, 0, 0, vec![0]), None, vec![1, 2, 3]),
            Node::new(1, Token::new(TokenType::AndSym, 0, 0, vec![0]), Some(0), vec![4, 5]),
            Node::new(2, Token::new(TokenType::As, 0, 0, vec![0]), Some(0), vec![]),
            Node::new(3, Token::new(TokenType::Asterisk, 0, 0, vec![0]), Some(0), vec![]),
            Node::new(4, Token::new(TokenType::Boolean, 0, 0, vec![0]), Some(2), vec![]),
            Node::new(5, Token::new(TokenType::BooleanType, 0, 0, vec![0]), Some(2), vec![]),
            ];
            
    for i in 0..nodes.len(){
        ast.add(&nodes[i])
    }

    ast.print()

}
