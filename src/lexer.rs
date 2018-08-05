use std::str::Chars;
use std::iter::Peekable;
use token::{self, Token};

lazy_static! {
    static ref SKIP_CHARS: Vec<char> = {
        vec![' ', '\t', '\n', '\r']
    };
}

pub struct Lexer<'a> {
    iter: Peekable<Chars<'a>>,
    curr_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        let mut lxr = Lexer {
            iter: input.chars().peekable(),
            curr_char: None,
        };
        lxr.read_char();
        lxr
    }

    fn read_char(&mut self) {
        self.curr_char = self.iter.next();
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.curr_char {
            Some(ch) => {
                match ch {
                    '=' => {
                        let is_equal = self.peek_char().map_or(false, |c| *c == '=');

                        if is_equal {
                            self.read_char();
                            Token::Equal
                        } else {
                            Token::Assign
                        }
                    }
                    '!' => {
                        let is_notequal = self.peek_char().map_or(false, |c| *c == '=');

                        if is_notequal {
                            self.read_char();
                            Token::NotEqual
                        } else {
                            Token::Bang
                        }
                    }
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '*' => Token::Asterisk,
                    '/' => Token::Slash,
                    ';' => Token::SemiColon,
                    '(' => Token::LParen,
                    ')' => Token::RParen,
                    '{' => Token::LBrace,
                    '}' => Token::RBrace,
                    ',' => Token::Comma,
                    '<' => Token::LessThan,
                    '>' => Token::GraterThan,
                    _ => {
                        if Lexer::is_letter(ch) {
                            let ident = self.read_identifier();
                            return token::lookup_ident(&ident);
                        } else if Lexer::is_digit(ch) {
                            return Token::Integer(self.read_number());
                        } else {
                            Token::Illegal
                        }
                    }
                }
            }
            None => Token::EoF,
        };

        self.read_char();
        token
    }

    fn is_letter(ch: char) -> bool {
        'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
    }

    fn is_digit(ch: char) -> bool {
        '0' <= ch && ch <= '9'
    }

    fn read_identifier(&mut self) -> String {
        let mut ident = String::new();
        while self.curr_char.map_or(false, |c| Lexer::is_letter(c)) {
            ident.push(self.curr_char.unwrap());
            self.read_char();
        }
        ident
    }

    fn read_number(&mut self) -> String {
        let mut number = String::new();
        while self.curr_char.map_or(false, |c| Lexer::is_digit(c)) {
            number.push(self.curr_char.unwrap());
            self.read_char();
        }
        number
    }

    fn skip_whitespace(&mut self) {
        while self.curr_char.map_or(false, |c| SKIP_CHARS.contains(&c)) {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod text_lexer {
    use lexer::Lexer;
    use token::Token::*;

    #[test]
    fn test_next_token() {
        let input = r#"let five = 5;
        let ten = 10;
        
        let add = fn(x, y) {
            x + y;
        };
        
        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        100 == 100;
        8!= 9;"#;

        let tests = vec![
            Let,
            Identifier(String::from("five")),
            Assign,
            Integer(String::from("5")),
            SemiColon,
            Let,
            Identifier(String::from("ten")),
            Assign,
            Integer(String::from("10")),
            SemiColon,
            Let,
            Identifier(String::from("add")),
            Assign,
            Function,
            LParen,
            Identifier(String::from("x")),
            Comma,
            Identifier(String::from("y")),
            RParen,
            LBrace,
            Identifier(String::from("x")),
            Plus,
            Identifier(String::from("y")),
            SemiColon,
            RBrace,
            SemiColon,
            Let,
            Identifier(String::from("result")),
            Assign,
            Identifier(String::from("add")),
            LParen,
            Identifier(String::from("five")),
            Comma,
            Identifier(String::from("ten")),
            RParen,
            SemiColon,
            Bang,
            Minus,
            Slash,
            Asterisk,
            Integer(String::from("5")),
            SemiColon,
            Integer(String::from("5")),
            LessThan,
            Integer(String::from("10")),
            GraterThan,
            Integer(String::from("5")),
            SemiColon,
            If,
            LParen,
            Integer(String::from("5")),
            LessThan,
            Integer(String::from("10")),
            RParen,
            LBrace,
            Return,
            True,
            SemiColon,
            RBrace,
            Else,
            LBrace,
            Return,
            False,
            SemiColon,
            RBrace,
            Integer(String::from("100")),
            Equal,
            Integer(String::from("100")),
            SemiColon,
            Integer(String::from("8")),
            NotEqual,
            Integer(String::from("9")),
            SemiColon,
            EoF,
        ];

        let mut lexer = Lexer::new(input);

        for tt in tests.iter() {
            let token = lexer.next_token();
            assert_eq!(&token, tt);
        }
    }
}
