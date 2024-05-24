/*
THE DOLLSCRIPT INTERPRETER 
by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// All the types of tokens that
/// "Dollscript" knows.
pub enum TokenType {
    InspoImport,
    SlayyPublic,
    BagStruct,
    UserEntityName,
    OpenCurly,
    CloseCurly,
    Colon,
    CashInteger,
    WisdomString,
    SmartFloat,
    ShopFunction,
    HeartComment,
    OpenParen,
    CloseParen,
    Number,
    UserString,
    LetMutable,
    LawImmutable,
    EqualsAssign,
    ForKeyword,
    ReturnBuy,
    SemiColon,
    Comma,
    InKeyword,
    PlusOperation,
    TimesOperation,
    DivisionOperation,
    MinusOperation,
    SleepVoid
}

pub struct Token {
    pub token_type: TokenType,
    pub token_value: String,
    pub start_pos: usize,
    pub end_pos: usize
}

impl Token {
    pub fn new(
        token_type: &TokenType,
        token_value: &str,
        start_pos: &usize,
        end_pos: &usize
    ) -> Token {
        Token {
            token_type: token_type,
            token_value: token_value.to_string(),
            start_pos: start_pos.to_owned(),
            end_pos: end_pos.to_owned()
        }
    }
}

pub struct Lexer {
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Lexer {
        let mut token_array: Vec<Token> = Vec::new();
        Lexer { tokens: token_array }
    }
}
