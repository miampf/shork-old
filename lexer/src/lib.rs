pub mod tokens;
pub mod lexer;

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::tokens::TokenType::*;
    use shork_error::report::{StderrReporter, Reporter};

    #[test]
    fn lexing_keywords() {
        let source = "integer float char string boolean regex T reef get from as define and or for while do in if else return structure implement private";
        let mut r = StderrReporter::new();
        let mut l = Lexer::new(source.to_string(), &mut r);
        let res = l.scan_tokens();
        if res.is_err(){
            let r = StderrReporter::new();
            r.display_error(res.unwrap_err());
            assert_eq!(1, 2)
        }

        let mut v = Vec::new();
        for t in l.get_tokens(){
            v.push(t.token_type().clone())
        }
        assert_eq!(v, vec![
            Integer, Float, Char, String, Boolean, Regex, T, Reef, Get,
            From, As, Define, And, Or, For, While, Do, In, If, Else,
            Return, Structure, Implement, Private, Eof
        ])
    }

    #[test]
    fn lexing_vars() {
        let source = "test = \"Hello\" test2 = 44 test_3 = 12.2 Test4 = false test5uwu = #iamregex#";
        let mut r = StderrReporter::new();
        let mut l = Lexer::new(source.to_string(), &mut r);
        let res = l.scan_tokens();
        if res.is_err(){
            let r = StderrReporter::new();
            r.display_error(res.unwrap_err());
            assert_eq!(1, 2)
        }

        let mut v = Vec::new();
        for t in l.get_tokens(){
            v.push(t.token_type().clone())
        }
        assert_eq!(v, vec![
            Identifier, Equal, StringType, Identifier, Equal, IntegerType,
            Identifier, Equal, FloatType, Identifier, Equal, BooleanType,
            Identifier, Equal, RegexType, Eof
        ])
    }

    #[test]
    fn lexing_newlines() {
        let source = r#"reef asdf
        iamgroot
        seemstowork()
        nonewline{
            newline]
        "#;
        let mut r = StderrReporter::new();
        let mut l = Lexer::new(source.to_string(), &mut r);
        let res = l.scan_tokens();
        if res.is_err(){
            let r = StderrReporter::new();
            r.display_error(res.unwrap_err());
            assert_eq!(1, 2)
        }

        let mut v = Vec::new();
        for t in l.get_tokens(){
            v.push(t.token_type().clone())
        }
        assert_eq!(v, vec![
            Reef, Identifier, NewLine, Identifier, NewLine, Identifier,
            LeftParen, RightParen, NewLine, Identifier, LeftCurlyParen,
            Identifier, RightSquareParen, NewLine, Eof
        ])
    }
}