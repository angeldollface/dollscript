#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub enum NumberVariable {
    TokenType::VarKeyword,
    TokenType::VarName,
    TokenType::Assign,
    TokenType::NumberLiteral
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub enum StringVariable {
    TokenType::VarKeyword,
    TokenType::VarName,
    TokenType::Assign,
    TokenType::StringLiteral
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub enum Grammar {
    NumberVariable,
    StringVariable
}

pub struct Statement {
    tokens: Vec<Token>,
    statement_type: Grammar
}

impl Statement {
    pub fn new(
        tokens: &Vec<Token>,
        statement_type: &Grammar
    ) -> Statement {
        return Statement {
            tokens: tokens.to_vec(),
            statement_type: statement_type.to_owned()
        }
    }
    pub fn to_string(&self) -> String {
        let mut token_strings: Vec<String> = Vec::new();
        for token in self.tokens {
            token_strings.push(token.to_string());
        }
        let tokens_string: String = token_strings.join("");
        return format!("{:?} :\n\t{}", &self.statement_type, tokens_string);
    }
}

pub fn parse(src: &String) -> Vec<Statement> {
    let mut result: Vec<Statement> = Vec::new();
    let lexed: Vec<Token> = Vec::new();    
    return result;
}
