//! Evaluator for meta-language. 

use sexp;
use sexp::Sexp::{ List, Atom };
use sexp::Atom::S;
use std::collections::HashMap;
use std::result::Result;
use lang::parser::*;
use lang::compilation::*;
use std::string::String;

pub type VSymbol = String;
type EvalRes = Result<QEval,String>;


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

fn syntax_err(detail:Option<&str>) -> EvalRes {
    match detail {
        Some(q) => {
            let s = format!("Syntax error: {}", q);
            Err(s)
        },
        None => Err(String::from("Syntax error")),
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

    fn eval_module(&mut self, s: &sexp::Sexp) -> EvalRes {
        match s {
            &Atom(ref a) => syntax_err(Some("atom unexpected")), //Err("Parser error: not expecting atom."),
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
                            Some(x) => Ok(QEval::Nothing),
                            None => syntax_err(Some("nothing defined")),
                        }
                    },
                    None => syntax_err(Some("Invalid module name.")),
                }
            }
        }
    }

    fn compile_instruction(&mut self, s: &sexp::Sexp) -> Vec<IlGate> {
        let mut qcc = QCompiler::new(&self);
        qcc.compile(s)
    }

    fn eval_qdef(&mut self, s: &sexp::Sexp) -> EvalRes {
        match s {
            &Atom(ref a) => syntax_err(Some("qdef")),
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
                                            Ok(QEval::S(format!("Defined bit at {}", mem_addr)))
                                        },
                                        _ => syntax_err(None),
                                    },
                                    _ => syntax_err(None),
                                }
                            } else if &typestr[..] == "lambda" {
                                let mut res:Vec<Vec<IlGate>> = vec![];
                                let arg_list:Vec<VSymbol> = match &v[2] {
                                    &Atom(_) => vec![],
                                    &List(ref vargs) => {
                                        /*let mut lst:Vec<String> = vec![];
                                        for x in vargs {
                                            lst.push(x)
                                        }
                                        lst*/
                                        vargs.into_iter().map(|x| { match x {
                                            &Atom(ref a) => match a {
                                                &S(ref z) => z.clone(),
                                                _ => String::from(""),
                                            },
                                            _ => String::from(""),
                                        } }).collect()
                                    }
                                };
                                println!("Arg list: {:?}\n", arg_list);
                                match &v[3] {
                                    &Atom(_) => syntax_err(Some("near lambda")),
                                    &List(ref vec) => {
                                        for x in vec.iter() {
                                            res.push(self.compile_instruction(x))
                                        }
                                        println!("Instructions: {:?}\n", res);
                                        Ok(QEval::S(String::from("hi")))
                                    }
                                }
                            } else {
                                syntax_err(None)
                            }
                        },
                        _ => syntax_err(None), 
                    },
                    _ => syntax_err(None), 
                }
            }
        }
    }

    pub fn eval(&mut self, s: &sexp::Sexp) -> EvalRes {
        match s {
            &Atom(ref a) => { syntax_err(None) },
            &List(ref v) => {
                if is_sexp_atom_s(&v[0], "module") {
                    self.eval_module(s)
                } else if is_sexp_atom_s(&v[0], "qdef") {
                    self.eval_qdef(s)
                } else {
                    syntax_err(Some("top level definition must be a module"))
                }
            },
        }
    }

    pub fn evaluate(&mut self) -> EvalRes {
        match &self.parser.ast {
            &Some(ref tree) => self.eval(tree),
            _ => Err(String::from("Parser failed to create S-expression structure. Find your missing paren.")), 
        }
    }
}

