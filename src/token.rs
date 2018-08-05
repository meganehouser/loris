use std::fmt::{Display, Formatter, Result};
use std::collections::HashMap;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Token> ={
        let mut keywords = HashMap::new();
        keywords.insert("let", Token::Let);
        keywords.insert("fn", Token::Function);
        keywords.insert("if", Token::If);
        keywords.insert("else", Token::Else);
        keywords.insert("return", Token::Return);
        keywords.insert("true", Token::True);
        keywords.insert("false", Token::False);
        keywords
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Let,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Identifier(String),
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LessThan,
    GraterThan,
    Integer(String),
    Comma,
    SemiColon,
    Function,
    Assign,
    EoF,
    Illegal,
    If,
    Else,
    Return,
    True,
    False,
    Equal,
    NotEqual,
}

pub fn lookup_ident(ident: &str) -> Token {
    if let Some(tkn) = KEYWORDS.get(ident) {
        tkn.clone()
    } else {
        Token::Identifier(ident.to_string())
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Token::Let => write!(f, "LET"),
            Token::LParen => write!(f, "LEFT PAREN"),
            Token::RParen => write!(f, "RIGHT PAREN"),
            Token::LBrace => write!(f, "LEFT BRACE"),
            Token::RBrace => write!(f, "RIGHT BRACE"),
            Token::Identifier(ref ident) => write!(f, "IDENTIFIER ({})", ident),
            Token::Plus => write!(f, "PLUS"),
            Token::Minus => write!(f, "MINUS"),
            Token::Bang => write!(f, "BANG"),
            Token::Asterisk => write!(f, "ASTERISK"),
            Token::Slash => write!(f, "SLASH"),
            Token::LessThan => write!(f, "LESS THAN"),
            Token::GraterThan => write!(f, "GRATER THAN"),
            Token::Integer(ref no) => write!(f, "INTEGER ({})", no),
            Token::Comma => write!(f, "COMMA"),
            Token::SemiColon => write!(f, "SEMICOLON"),
            Token::Function => write!(f, "FUNCTION"),
            Token::Assign => write!(f, "ASSIGN"),
            Token::EoF => write!(f, "EOF"),
            Token::Illegal => write!(f, "ILLEGAL"),
            Token::If => write!(f, "IF"),
            Token::Else => write!(f, "ELSE"),
            Token::Return => write!(f, "RETURN"),
            Token::True => write!(f, "TRUE"),
            Token::False => write!(f, "FALSE"),
            Token::Equal => write!(f, "EQUAL"),
            Token::NotEqual => write!(f, "NOT EQUAL"),
        }
    }
}
