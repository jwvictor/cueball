//! Compiler for "quantum lambdas."

use lang::parser::*;
use lang::evaluator::*;
use sexp;
use sexp::Sexp::{ List, Atom };
use sexp::Atom::S;
use std::collections::HashMap;

/// IL gate type enumeration 
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
    pub args: Vec<f64>,
}

/// Quantum lambda IL
#[derive(Debug, Clone)]
pub struct IlLambda {
    pub formal_params:Vec<String>,
    pub gates:Vec<IlGate>,
}

/// Reference to quantum data
#[derive(Debug, Clone)]
pub struct QRef {
    pub mem: Option<u64>,
    pub qref: Option<Box<QRef>>,
}

/// Quantum subroutine compiler 
#[derive(Debug, Clone)]
pub struct QCompiler<'a> {
    evaluator: &'a Evaluator<'a>,
    instructions: Vec<IlGate>,
    v_stack: Vec<HashMap<VSymbol,QRef>>,
}

impl QRef {
    pub fn from_qptr(qbit: u64) -> QRef {
        QRef { mem: Some(qbit), qref: None }
    }
    pub fn from_qref(qref: &QRef) -> QRef {
        QRef { mem: None, qref: Some(Box::new(qref.clone())) }
    }
}

impl <'a> QCompiler<'a> {
    pub fn new(evalr:&'a Evaluator) -> QCompiler<'a> {
        QCompiler { evaluator: evalr, instructions: vec![], v_stack: vec![] }
    }

    pub fn resolve_symbol(&mut self, sym: VSymbol) -> Option<u64> {
        None 
    }

    pub fn compile(&mut self, instr: &sexp::Sexp) -> Vec<IlGate> {
        println!("Compiling instruction: {:?}\n", instr);
        match instr {
            &List(ref v) => {
                match &v[0] {
                    &Atom(ref q) => match q {
                        &S(ref s) => {
                            if s == "h" {
                                println!("Hadamard!")
                            }
                            vec![]
                        },
                        _ => vec![],
                    },
                    _ => vec![],
                }
            },
            _ => vec![],
        }
    }
}
