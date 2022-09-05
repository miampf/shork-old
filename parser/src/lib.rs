pub mod tree;
pub mod expressions;

#[cfg(test)]
mod tests {
    use crate::tree::*;
    use shork_lexer::tokens::{Token, TokenType};

    #[test]
    fn tree_ast() {
        let mut ast = AST::new();

        // array is unsorted to test search
        let nodes = [
            Node::new(2, Token::new(TokenType::And, 0, 0, vec![0]), Some(0), vec![4, 5]),
            Node::new(5, Token::new(TokenType::And, 0, 0, vec![0]), Some(2), vec![]),
            Node::new(3, Token::new(TokenType::And, 0, 0, vec![0]), Some(0), vec![4, 5]),
            Node::new(0, Token::new(TokenType::And, 0, 0, vec![0]), None, vec![1, 2, 3]),
            Node::new(1, Token::new(TokenType::And, 0, 0, vec![0]), Some(0), vec![4, 5]),
            Node::new(4, Token::new(TokenType::And, 0, 0, vec![0]), Some(2), vec![]),
            ];
            
        for n in nodes.clone(){
            ast.add(n)
        }

        let n_by_id = ast.get(4).unwrap();
        assert_eq!(n_by_id, &nodes[5]);

        let n_siblings = ast.siblings(&nodes[1]).unwrap();
        assert_eq!(vec![4, 5], n_siblings)
    }
}
