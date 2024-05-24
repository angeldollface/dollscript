/*
THE DOLLSCRIPT INTERPRETER 
by Alexander Abraham 
a.k.a. "Angel Dollface".
Licensed under the DSL v1.
*/

/// All the types of tokens that
/// "Dollscript" knows.
#[derive(Debug, Clone)]
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

/// A data structure to hold all information
/// on a token lexed in Dollscript code with
/// all fields kept public.
pub struct Token {
    pub token_type: TokenType,
    pub token_value: String,
    pub start_pos: usize,
    pub end_pos: usize
}

/// Implementing methods for the
/// "Token" structure.
impl Token {

    /// Implementing the generic "new"
    /// method to create a new instance of
    /// this data structure.
    pub fn new(
        token_type: &TokenType,
        token_value: &str,
        start_pos: &usize,
        end_pos: &usize
    ) -> Token {
        Token {
            token_type: token_type.to_owned(),
            token_value: token_value.to_string(),
            start_pos: start_pos.to_owned(),
            end_pos: end_pos.to_owned()
        }
    }

}

/// A data structure to tokenize and
/// analyze Dollscript code.
pub struct Lexer {
    pub tokens: Vec<Token>,
}

/// Implementing methods
/// for the "Lexer" data structure.
impl Lexer {

    /// A function to create an empty new instance
    /// of the "Lexer" data structure.
    pub fn new() -> Lexer {
        let mut token_array: Vec<Token> = Vec::new();
        Lexer { tokens: token_array }
    }

}
