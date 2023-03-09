/* {
    type: "KEYWORD",
    value: "const"
}
---
{
    type: "OPERATOR",
    value: "+"
}
---
{
    type: "NUMBER",
    value: "2"
}*/
#[derive(Debug,PartialEq)]
pub enum TokenType {
    ENDLINE,
    L_CURLY,
    R_CURLY,
    L_PAREN,
    R_PAREN,
    IDENTIFIER,
    NUMBER,
    OPERATOR,
    KEYWORD,
    SHAPE_KEYWORD,
    SHIFT_KEYWORD,
    STRETCH_KEYWORD,
    ROTATE_KEYWORD,
    EVOLVE_KEYWORD,
    PROPERTIES, 
}


#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Token {
        Token {
            token_type: token_type,
            value: value,
        }
    }
}