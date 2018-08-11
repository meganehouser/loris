use std::fmt::{Display, Formatter, Result};
use token;

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program { statements: Vec::new() }
    }
}

#[derive(Debug)]
pub enum Statement {
    Let {
        token: token::Token,
        name: String,
        value: Option<Box<Expression>>,
    },
    Return {
        token: token::Token,
        value: Option<Box<Expression>>,
    },
    Illegal,
}

#[derive(Debug)]
pub enum Expression {
    Identifier { token: token::Token, value: String },
}

impl Display for Statement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Statement::Let {
                token: t,
                name: n,
                value: v,
            } => write!(f, "Let(token:{}, name:{}, value:)", t, n),//", v.unwrap()),
            Statement::Return { token: t, value: v } => write!(f, "Return(token:{}, value:)", t), //, v)
            Illegal => write!(f, "Illegal()"),
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Expression::Identifier { token: t, value: v } => {
                write!(f, "Let(token:{}, value:{})", t, v)
            }
        }
    }
}
//impl Node for Statement {
//    pub fn token_literal(&self) -> String {
//        format!("{}", self.token)
//    }
//}
//
//
//impl Program {
//pub fn token_literal(&self) -> String {
//    if self.statements.len() > 0 {
//        p.statements[0].token_literal()
//    } else {
//        String::from("")
//    }
//}
