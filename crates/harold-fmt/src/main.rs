use std::fmt::Write;
use std::fs;
use std::path::PathBuf;

use clap::Parser;

use harold_ir::*;

/// Pretty-printer for Harold assembly language.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the Harold assembly file to format in place.
    path: PathBuf,
}

fn format(input: &str, out: &mut impl Write) -> Result<(), std::fmt::Error> {
    let document = match harold_ir::asm_parser::document(input) {
        Ok(document) => document,
        Err(_) => {
            return Err(std::fmt::Error);
        }
    };
    let mut indent: usize = 0;

    for statement in document.statements.iter() {
        match statement {
            Statement::Instruction(instruction) => {
                for _ in 0..indent {
                    write!(out, "    ")?;
                }

                write!(out, "{} ", instruction.mnemonic)?;

                for (index, operand) in instruction.operands.iter().enumerate() {
                    match operand {
                        Operand::Register(name) => {
                            write!(out, "{}", name)?;
                        }
                        Operand::Integer(value) => {
                            write!(out, "{}", value)?;
                        }
                        Operand::Float(value) => {
                            write!(out, "{}", value)?;
                        }
                    }
                    if index < instruction.operands.len() - 1 {
                        write!(out, ", ")?;
                    }
                }

                writeln!(out)?;
            }
            Statement::Label(label) => {
                indent = indent.saturating_sub(1);
                indent += 1;
                writeln!(out, "{}:", label.name)?;
            }
            _ => {
                eprintln!("ERROR");
            }
        }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();

    let input = match fs::read_to_string(&args.path) {
        Ok(input) => input,
        Err(err) => {
            eprintln!("ERROR: failed to read {}: {}", args.path.display(), err);
            std::process::exit(1);
        }
    };

    let mut output = String::new();
    if format(&input, &mut output).is_err() {
        eprintln!("ERROR: failed to parse document");
        std::process::exit(1);
    }

    if let Err(err) = fs::write(&args.path, output) {
        eprintln!("ERROR: failed to write {}: {}", args.path.display(), err);
        std::process::exit(1);
    }
}
