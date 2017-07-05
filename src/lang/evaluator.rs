//! Evaluator for meta-language. 

use sexp;
use sexp::Sexp::{ List, Atom };
use sexp::Atom::S;
use std::collections::HashMap;
use std::result::Result;
use lang::parser::*;
use std::string::String;

type VSymbol = String;

/// IL gate 
#[derive(Debug, Clone, Copy)]
pub enum IlGateType {
    HADAMARD,
    PAULIX,
    PAULIY,
    PAULIZ,
    CNOT,
}

/// IL circuit application 
#[derive(Debug, Clone)]
pub struct IlGate {
    pub gate_type: IlGateType,
    pub bit0: Option<VSymbol>,
    pub bit1: Option<VSymbol>,
    pub bit2: Option<VSymbol>,
}

/// Quantum lambda IL
#[derive(Debug, Clone)]
pub struct IlLambda {
    pub gates:Vec<IlGate>,
}

/// Quantum reference 
#[derive(Debug, Clone)]
pub struct QRef {
    pub mem: Option<u64>,
    pub qref: Option<Box<QRef>>,
}

/// Evaluator 
#[derive(Debug, Clone)]
pub struct Evaluator<'a> {
    pub parser: &'a Parser<'a>,
    pub mmap: HashMap<VSymbol, u64>,
    pub qrefs: HashMap<VSymbol, QRef>,
    pub qlambdas: HashMap<VSymbol, IlLambda>,
    pub name: Option<String>,
}

fn is_sexp_atom_s(a:&sexp::Sexp, c:&str) -> bool {
    match *a {
        Atom(ref q) => { match q {
            &S(ref s) => (s == c),
            _ => false,
        } },
        _ => false,
    }
}

impl <'a> Evaluator<'a> {

    pub fn new<'b>(parser:&'b Parser) -> Evaluator<'b> {
        Evaluator {
            parser: parser,
            mmap: HashMap::new(),
            qlambdas: HashMap::new(),
            qrefs: HashMap::new(),
            name: None, 
        }
    }

    fn wrap_up(&mut self, s:&[sexp::Sexp]) -> Vec<sexp::Sexp> {
        s.to_vec()
    }

    fn eval_module(&mut self, s: &sexp::Sexp) -> Result<QEval,&'static str> {
        match s {
            &Atom(ref a) => Err("Parser error: not expecting atom."),
            &List(ref v) => {
                let mod_name = match &v[1] {
                    &Atom(ref a) => match a {
                        &S(ref s) => Some((*s).clone()),
                        _ => None,
                    },
                    _ => None,
                };
                match mod_name {
                    Some(mod_n) => {
                        self.name = Some(mod_n);
                        let q = self.wrap_up(&v[2..]);
                        let qs = format!("qs = {:?}", q);
                        println!("QS = {:?}\n", qs);
                        let ss = format!("{} \n{}\n", s, qs);
                        let mut r:Option<QEval> = None;
                        for i in 2..v.len() {
                            match self.eval(&v[i]) {
                                Ok(z) => r = Some(z),
                                Err(q) => (),
                            }
                        }
                        match r {
                            Some(x) => Ok(x),
                            None => Err("Nothing defined in module"),
                        }
                    },
                    None => Err("Invalid module name."),
                }
                /*;
                match &v[2] {
                    &Atom(_) => Err("Syntax error in module."),
                    &List(ref v) => {
                        if v[0] == "qdef" {
                            eval_qdef(List(&v
                    }*/
            }
        }
    }

    fn eval_qdef(&mut self, s: &sexp::Sexp) -> Result<QEval,&'static str> {
        match s {
            &Atom(ref a) => Err("Syntax error: qdef"),
            &List(ref v) => {
                match &v[1] {
                    &Atom(ref x) => match x {
                        &S(ref s) => {
                            let typestr = s;
                            if &typestr[..] == "bit" {
                                let mem_addr = self.mmap.len();
                                match &v[2] {
                                    &Atom(ref x) => match x {
                                        &S(ref nn) => {
                                            let qn = nn;
                                            self.mmap.insert((*qn).clone(), mem_addr as u64);
                                            Ok(format!("Defined bit at {}", mem_addr))
                                        },
                                        _ => Err("Syntax"),
                                    },
                                    _ => Err("Bad syntax"),
                                }
                            } else if &typestr[..] == "lambda" {
                                Ok(String::from("hi"))
                            } else {
                                Err("Syntax error")
                            }
                        },
                        _ => Err("Syntax error"),
                    },
                    _ => Err("Syntax error"),
                }
            }
        }
        //Ok(format!("{}", s))
    }

    pub fn eval(&mut self, s: &sexp::Sexp) -> Result<QEval,&'static str> {
        match s {
            &Atom(ref a) => { Err("Parse error") },
            &List(ref v) => {
                if is_sexp_atom_s(&v[0], "module") {
                    self.eval_module(s)
                } else if is_sexp_atom_s(&v[0], "qdef") {
                    self.eval_qdef(s)
                } else {
                    Err("Syntax: top level definition must be a module.")
                }
            },
        }
    }

    pub fn evaluate(&mut self) -> Result<QEval, &'static str> {
        match &self.parser.ast {
            &Some(ref tree) => self.eval(tree),
            _ => Err("Parse error"),
        }
    }
}

