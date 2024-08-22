use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String {
        // TODO: check book for what actually needs to be implemented
        return String::from("");
    }
    fn to_string(&self) -> String;
}

#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

#[derive(Debug)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
}

impl Expression {
    fn to_string(&self) -> String {
        let mut out = String::new();
        match self {
            Expression::Identifier(ident) => out.push_str(&ident.to_string()),
            Expression::Literal(lit) => out.push_str(&lit.to_string()),
        };
        return out.clone();
    }
}

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            let statement = &self.statements[0];
            match statement {
                Statement::Let(ls) => ls.token_literal().clone(),
                Statement::Return(rs) => rs.token_literal().clone(),
                Statement::Expression(exp) => exp.token_literal().clone(),
                _ => unimplemented!("[ERROR]: Unimplemented statement type"),
            }
        } else {
            todo!();
        }
    }

    fn to_string(&self) -> String {
        let mut out = String::new();
        for statement in &self.statements {
            match statement {
                Statement::Let(ls) => {
                    out.push_str(&ls.to_string());
                }
                Statement::Return(rs) => {
                    out.push_str(&rs.to_string());
                }
                Statement::Expression(exp) => {
                    out.push_str(&exp.to_string());
                }
                _ => unimplemented!("[ERROR]: Unimplemented statement type"),
            }
        }
        return out.clone();
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,  // Left side value of the let statement
    pub value: Expression, // Right side value of the let statement
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }

    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str(self.token_literal().as_str());
        out.push_str(" ");
        out.push_str(&self.name.to_string());
        out.push_str(" = ");

        match &self.value {
            Expression::Identifier(ident) => out.push_str(&ident.to_string()),
            Expression::Literal(lit) => out.push_str(&lit.to_string()),
            _ => unimplemented!("[ERROR]: Unimplemented expression type"),
        };
        out.push_str(";");
        return out.clone();
    }
}

#[derive(Debug)]
pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Expression,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }

    fn to_string(&self) -> String {
        let mut out = String::new();
        out.push_str("return ");
        match &self.return_value {
            Expression::Identifier(ident) => out.push_str(&ident.to_string()),
            Expression::Literal(lit) => out.push_str(&lit.to_string()),
        };
        out.push_str(";");
        return out.clone();
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        return self.token.literal.clone();
    }

    fn to_string(&self) -> String {
        return self.expression.to_string();
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
}

impl Node for Identifier {
    fn to_string(&self) -> String {
        return self.name.clone();
    }
}

#[derive(Debug)]
pub struct Literal {
    pub value: String,
}

impl Node for Literal {
    fn to_string(&self) -> String {
        return self.value.clone();
    }
}

// impl Program {
//     pub fn token_literal(&self) -> String {
//         if self.statements.len() > 0 {
//             return self.statements[0].token_literal();
//         } else {
//             return String::from("");
//         }
//     }
//
// }

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_to_string_1() {
        let program = Program {
            statements: vec![
                Statement::Let(LetStatement {
                    token: Token {
                        kind: crate::token::TokenType::LET,
                        literal: "let".to_string(),
                    },
                    name: Identifier {
                        name: "myVar".to_string(),
                    },
                    value: Expression::Identifier(Identifier {
                        name: "anotherVar".to_string(),
                    }),
                }),
                Statement::Return(ReturnStatement {
                    token: Token {
                        kind: crate::token::TokenType::RETURN,
                        literal: "return".to_string(),
                    },
                    return_value: Expression::Literal(Literal {
                        value: "10".to_string(),
                    }),
                }),
            ],
        };

        assert_eq!(program.to_string(), "let myVar = anotherVar;return 10;");
    }
}
