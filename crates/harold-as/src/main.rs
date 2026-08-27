use std::collections::HashMap;

use harold_ir::*;

// Abstract Syntax Tree (AST)

// Source Code -> AST -> Analysis -> Emit
// Analysis -> Lexical Analysis (parsing) -> Semantic Analysis (verification)

struct Label {
    offset: usize,
}

fn main() {
    println!("harold-as");

    let input = include_str!("../../../examples/fib.asm");
    let result = asm_parser::document(input).unwrap();

    let mut labels: HashMap<String, Label> = HashMap::new();
    let mut bytecode: Vec<u8> = vec![];

    for statement in result.statements.iter() {
        match statement {
            Statement::Comment(_) => {}
            Statement::Label(label) => {}
            Statement::Instruction(instruction) => {
                // TODO: compile
                let op_1 = instruction.operands.get(0).unwrap_or(&Operand::None);
                let op_2 = instruction.operands.get(1).unwrap_or(&Operand::None);
                let instruction = (instruction.mnemonic.as_str(), op_1, op_2);

                match instruction {
                    ("mov", Operand::Register(register), Operand::Integer(data)) => {
                        // TODO: is this correct way to encode signed 8-bit in binary in Rust?
                        let data: u8 = *data as i8 as u8;

                        match register.as_str() {
                            "rx" => {
                                bytecode.push(0x06);
                                bytecode.push(data);
                            }
                            "ry" => {
                                bytecode.push(0x07);
                                bytecode.push(data);
                            }
                            "rv" => {
                                bytecode.push(0x0e);
                                bytecode.push(data);
                            }
                            _ => {
                                eprintln!("unrecognized register: {}", register);
                                std::process::exit(1);
                            }
                        }
                    }
                    ("mov", Operand::Register(dest), Operand::Register(src)) => {
                        match (dest.as_str(), src.as_str()) {
                            ("rz", "rx") => {
                                bytecode.push(0x04);
                                bytecode.push(0x00);
                            }
                            ("ry", "rx") => {
                                bytecode.push(0x0b);
                                bytecode.push(0x00);
                            }
                            ("rx", "ry") => {
                                bytecode.push(0x0f);
                                bytecode.push(0x00);
                            }
                            ("rz", "ry") => {
                                bytecode.push(0x0c);
                                bytecode.push(0x00);
                            }
                            _ => {
                                eprintln!("unhandled mov {}, {}", dest, src);
                                std::process::exit(0);
                            }
                        }
                    }
                    ("print", Operand::None, Operand::None) => {
                        bytecode.push(0x03);
                        bytecode.push(0x00);
                    }
                    ("add", Operand::None, Operand::None) => {
                        bytecode.push(0x01);
                        bytecode.push(0x00);
                    }
                    ("dec", Operand::None, Operand::None) => {
                        bytecode.push(0x0f);
                        bytecode.push(0x00);
                    }
                    ("jg", Operand::Integer(addr), Operand::None) => {
                        bytecode.push(0x0a);
                        // TODO: is this correct way to encode signed 8-bit in binary in Rust?
                        bytecode.push(*addr as i8 as u8);
                    }
                    ("hlt", Operand::None, Operand::None) => {
                        bytecode.push(0x10);
                        bytecode.push(0x00);
                    }
                    _ => {
                        eprintln!("unrecognized instruction: {}", instruction.0);
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    // sprintf("hello: %d");

    println!("{:02x?}", bytecode);
    println!("{}", bytecode.len());

    std::fs::write("debug", bytecode).unwrap();

    // println!("{:?}", result);
}

// Backus-Naur Form / Extended Backus-Naur Form
// https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form

// parser generator (BNF, EBNF; PEG, LR, GLR, LALR / Regex)
// parser expression grammar (PEG)
// recursive-descent parsers
