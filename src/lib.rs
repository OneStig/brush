mod art;
mod ast;
mod error;
mod eval;
mod lexer;
mod parser;
mod tokens;

use crate::eval::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;

use crate::art::draw;
use crate::art::name;
//use crate::art::Circle;
use crate::art::Drawable;

use std::env;
use std::fs::File;
use std::io::Read;
use  std::fmt::Display;

use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;


use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
use std::string::String;

#[wasm_bindgen]
pub fn process_file(content: &str) -> String{
    process(content);
   let svg = name();
    svg
}

fn process(content: &str) {
    let mut lex = Lexer::new(String::from(content));
    let tokens = lex.lex();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();
    let mut interpreter = Interpreter::new(ast);
    interpreter.run();
}

  