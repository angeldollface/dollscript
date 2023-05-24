use regex::Regex;
use coutils::clean_split;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub enum TokenType {
    VarKeyword,
    Assign,
    NumberLiteral,
    StringLiteral,
    EndOfLine,
    VarName
}

#[derive(Clone)]
pub struct Token {
    pub name: TokenType,
    pub value: String
}

impl Token {
    pub fn new(
        name: &TokenType,
        value: &String
    ) -> Token {
        return Token {
            name: *name,
            value: value.to_owned()
        }
    }
    pub fn to_string(&self) -> String {
        return format!("{:?} : {}", &self.name, &self.value);
    }
}

pub fn patterns() -> HashMap<TokenType, Regex> {
    let mut result: HashMap<TokenType, Regex> = HashMap::new();
    result.insert(TokenType::VarKeyword, Regex::new(r"(LET)").unwrap());
    result.insert(TokenType::Assign, Regex::new(r"(=)").unwrap());
    result.insert(TokenType::NumberLiteral, Regex::new(r"([0-9]+)").unwrap());
    result.insert(TokenType::EndOfLine, Regex::new(r"(;)").unwrap());
    result.insert(TokenType::VarName, Regex::new(r"\$([A-Z]+)").unwrap());
    return result;
}

pub fn lex(src: &String) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let lines: Vec<String> = clean_split(src, &String::from("\n"));
    for line in lines {
        let char_list: Vec<String> = clean_split(&line, &String::from(""));
        let mut im_char_list: Vec<String> = Vec::new();
        for character in char_list {
            im_char_list.push(character);
            let joined_string: String = im_char_list.join("");
            for (k,v) in patterns().into_iter() {
                if v.is_match(&joined_string) {
                    im_char_list.clear();
                    let captured = v.captures(&joined_string).unwrap();
                    let new_token: Token = Token {
                        name: k,
                        value: captured.get(1).unwrap().as_str().to_string()
                    };
                    result.push(new_token);
                }
                else {
                    // Do nothing.
                }
            }
        }
    }
    return result;
}