
extern crate cueball;

use cueball::computer::QuantumComputer;
use cueball::algorithms::deutsch;
use cueball::gates;
use cueball::lang::parser;
use cueball::lang::evaluator;

fn main() {
    let mut c2 = QuantumComputer::new(1);
    c2.initialize(0);
    c2.apply(gates::hadamard(1));
    c2.collapse();
    let result = if 1 == c2.value() { "heads" } else { "tails" };
    println!("coin flip: {}", result);

    let rig:parser::Parser = parser::Parser::new("(module jasonmod (qdef bit my-thing) (qdef bit your-thing))");
    let mut evalr:evaluator::Evaluator = evaluator::Evaluator::new(&rig);
    let e = evalr.evaluate(); 
    println!("e: {:?}\n",  e);
    println!("evalr: {:?}\n",  evalr);
}
