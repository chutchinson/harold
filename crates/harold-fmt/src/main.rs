use std::fs::OpenOptions;
use std::io::prelude::*;

use harold_ir::*;

fn main() {
    let input = include_str!("../../../examples/debug.asm");
    let document = harold_ir::asm_parser::document(input).unwrap();
    let mut indent: usize = 0;

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open("debug_fmt.asm")
        .unwrap();

    for statement in document.statements.iter() {
        match statement {
            Statement::Instruction(instruction) => {
                for _ in 0..indent {
                    write!(file, "    ").unwrap();
                }

                write!(file, "{} ", instruction.mnemonic).unwrap();

                for (index, operand) in instruction.operands.iter().enumerate() {
                    match operand {
                        Operand::Register(name) => {
                            write!(file, "{}", name).unwrap();
                        }
                        Operand::Integer(value) => {
                            write!(file, "{}", value).unwrap();
                        }
                        Operand::Float(value) => {
                            write!(file, "{}", value).unwrap();
                        }
                    }
                    if index < instruction.operands.len() - 1 {
                        write!(file, ", ").unwrap();
                    }
                }

                writeln!(file).unwrap();
            }
            Statement::Label(label) => {
                indent += 1;
                writeln!(file, "{}:", label.name).unwrap();
            }
            _ => {
                eprintln!("ERROR");
            }
        }
    }
}
