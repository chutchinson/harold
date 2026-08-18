use harold_ir::*;

// Abstract Syntax Tree (AST)

// Source Code -> AST -> Analysis -> Emit

fn main() {
    println!("harold-as");

    let input = include_str!("../../../examples/debug.asm");
    let result = asm_parser::document(input).unwrap();

    // println!("{:?}", result);
}

// Backus-Naur Form / Extended Backus-Naur Form
// https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form

// parser generator (BNF, EBNF; PEG, LR, GLR, LALR / Regex)
// parser expression grammar (PEG)
// recursive-descent parsers
