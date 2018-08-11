use std::mem;
use ast;
use lexer::Lexer;
use token::Token;
use failure::{Error, Fail};

#[derive(Debug, Fail)]
#[fail(display = "expected next token {}, got {} insted", expected, next)]
pub struct ParserError {
    expected: Token,
    next: Token,
}

pub struct Parser<'a> {
    lxr: Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
    errors: Vec<ParserError>,
}

impl<'a> Parser<'a> {
    pub fn new(lxr: Lexer<'a>) -> Parser<'a> {
        let mut p = Parser {
            lxr: lxr,
            cur_token: Token::Illegal,
            peek_token: Token::Illegal,
            errors: Vec::new(),
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&mut self) {
        self.cur_token = mem::replace(&mut self.peek_token, self.lxr.next_token());
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program::new();

        while self.cur_token != Token::EoF {
            let stmt = self.parse_statement();
            program.statements.push(stmt);
            self.next_token();
        }

        program
    }

    fn parse_statement(&mut self) -> ast::Statement {
        match self.cur_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => ast::Statement::Illegal,
        }
    }

    fn push_err(&mut self, expected: Token) {
        let peek = self.peek_token.clone();
        self.errors.push(ParserError {
            expected: expected,
            next: peek,
        });
    }

    fn parse_let_statement(&mut self) -> ast::Statement {
        let name = if let Token::Identifier(ref ident) = self.peek_token {
            String::from(ident.as_str())
        } else {
            self.push_err(Token::Identifier("something".to_string()));
            return ast::Statement::Illegal;
        };

        self.next_token();

        if Token::Assign == self.peek_token {
            self.next_token();
        } else {
            self.push_err(Token::Assign);
            return ast::Statement::Illegal;
        }

        // ToDo: セミコロンまでスキップ
        while Token::SemiColon != self.cur_token {
            self.next_token();
        }

        ast::Statement::Let {
            token: Token::Let,
            name: name,
            value: None,
        }
    }

    fn parse_return_statement(&mut self) -> ast::Statement {
        // ToDo: セミコロンまでスキップ
        while Token::SemiColon != self.cur_token {
            self.next_token();
        }

        ast::Statement::Return {
            token: Token::Return,
            value: None,
        }
    }
}


#[cfg(test)]
mod parser_test {
    use ast::Statement;
    use token::Token;
    use lexer::Lexer;
    use parser::Parser;

    #[test]
    fn test_let_statements() {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 898989;
        "#;

        let lxr = Lexer::new(input);
        let mut prsr = Parser::new(lxr);

        let program = prsr.parse_program();
        check_parser_errors(&prsr);

        assert_eq!(program.statements.len(), 3);

        let tests = vec!["x", "y", "foobar"];
        for (expected, stmt) in tests.iter().zip(program.statements.iter()) {
            assert!(test_let_statement(expected, stmt));
        }
    }

    fn test_let_statement(name: &str, s: &Statement) -> bool {
        match s {
            Statement::Let {
                token: t,
                name: n,
                value: _v,
            } => (t == &Token::Let) && (n.as_str() == name),
            _ => false,
        }
    }

    #[test]
    fn test_return_statements() {
        let input = r#"
        return 5;
        return 10;
        return 999322;"#;

        let lxr = Lexer::new(input);
        let mut prsr = Parser::new(lxr);

        let program = prsr.parse_program();
        check_parser_errors(&prsr);

        for stmt in program.statements.iter() {
            if let Statement::Return { token: _, value: _ } = stmt {
            } else {
                panic!("not return");
            }
        }
    }

    fn check_parser_errors(parser: &Parser) {
        if parser.errors.len() > 0 {
            let message = parser.errors.iter().fold(String::new(), |mut m, err| {
                m.push_str(&format!("{}\n", err));
                m
            });
            panic!(message);
        }
    }
}
