//! Parser for meta-language. 

use sexp;
use sexp::Sexp::{ List, Atom };
use sexp::Atom::S;
use std::collections::HashMap;
use std::result::Result;
//use computer::QuantumComputer;
use std::string::String;

//pub type QEval = &'static str;
pub type QEval = String;

/// We panic if the initial state register has a different size to `width`.
pub fn parse_raw(s:&str) -> Option<sexp::Sexp> {
    match sexp::parse(s) {
        Ok(s) => Some(s),
        _ => None,
    }
}

/// Parser instance 
#[derive(Debug)]
pub struct Parser<'a> {
    pub raw: &'a str,
    pub success: bool,
    pub ast: Option<sexp::Sexp>,
}

/// Parser instance implementation 
impl <'a> Parser<'a> {
    /// New method for parser 
    pub fn new<'b>(raw_txt:&'b str) -> Parser<'b> {
        let ast = parse_raw(raw_txt);
        match ast {
            Some(asts) =>
                Parser {
                    raw: raw_txt,
                    success: true,
                    ast: Some(asts),
                },
            None =>
                Parser {
                    raw: raw_txt,
                    success: false,
                    ast: None,
                },
        }
    }
}

