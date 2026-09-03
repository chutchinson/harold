use std::fs;
use std::path::PathBuf;

use clap::Parser;

const RAM_SIZE: usize = 64 * 1024;

struct Cpu {
    memory: [u8; RAM_SIZE],
    rip: usize, // rip (register instruction pointer)
    rx: u16,
    ry: u16,
    rz: u16,
    rv: u16,
    halt: bool,
}

impl Cpu {
    pub fn new(program: &[u8]) -> Self {
        let mut memory = [0; RAM_SIZE];
        memory[..program.len()].copy_from_slice(program);

        Cpu {
            memory,
            rip: 0,
            rx: 0,
            ry: 0,
            rz: 0,
            rv: 0,
            halt: false,
        }
    }

    pub fn cycle(&mut self) {
        const STEP: usize = 4;

        // fetch-decode-execute
        if self.rip >= self.memory.len() {
            self.halt = true;
            return;
        }

        // fetch
        let opcode = self.memory[self.rip];
        let data: u16 =
            self.memory[self.rip + 1] as u16 | ((self.memory[self.rip + 2] as u16) << 8);
        let imm = self.memory[self.rip + 3];

        // decode
        match opcode {
            0x00 => {
                // NOP
                self.rip += STEP;
            }
            0x01 => {
                // ADD rx, ry -> rz
                self.rz = self.rx + self.ry;
                self.rip += STEP;
            }
            0x02 => {
                // SUB rx, ry -> rz
                self.rz = self.rx - self.ry;
                self.rip += STEP;
            }
            0x03 => {
                // PRINT rz
                println!("{}", self.rz);
                self.rip += STEP;
            }
            0x04 => {
                // MOV rx -> rz
                self.rz = self.rx;
                self.rip += STEP;
            }
            0x05 => {
                // MOV ry -> rz
                self.rz = self.ry;
                self.rip += STEP;
            }
            0x06 => {
                // MOV rx, value
                self.rx = data as u16;
                self.rip += STEP;
            }
            0x07 => {
                // MOV ry, value
                self.ry = data as u16;
                self.rip += STEP;
            }
            0x08 => {
                // JMP addr
                self.rip = data as usize;
            }
            0x09 => {
                // JZ addr
                if self.rz == 0 {
                    self.rip = data as usize;
                } else {
                    self.rip += STEP;
                }
            }
            0x0a => {
                // JG addr
                if self.rz > 0 {
                    self.rip = data as usize;
                } else {
                    self.rip += STEP;
                }
            }
            0x0b => {
                // MOV ry -> rx
                self.rx = self.ry;
                self.rip += STEP;
            }
            0x0c => {
                // MOV rz -> ry
                self.ry = self.rz;
                self.rip += STEP;
            }
            0x0d => {
                // MOV rv -> rz
                self.rz = self.rv;
                self.rip += STEP;
            }
            0x0e => {
                // MOV rv, value
                self.rv = data as u16;
                self.rip += STEP;
            }
            0x0f => {
                // DEC rv -> rz
                self.rv = self.rv.wrapping_sub(1);
                self.rz = self.rv;
                self.rip += STEP;
            }
            0x10 => {
                // HALT
                self.halt = true;
            }
            0x11 => {
                // mov [<addr>], <imm>
                self.memory[data as usize] = imm as u8;
                self.rip += STEP;
            }
            0x12 => {
                self.memory[self.rz as usize] = imm as u8;
                self.rip += STEP;
            }
            _ => {
                // Unknown instruction
                println!("Unknown instruction: {}", opcode);
                self.rip += STEP;
            }
        }
    }
}

/// Execute a Harold bytecode file.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the Harold bytecode file to execute.
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let program = match fs::read(&args.path) {
        Ok(program) => program,
        Err(err) => {
            eprintln!("ERROR: failed to read {}: {}", args.path.display(), err);
            std::process::exit(1);
        }
    };

    let mut cpu = Cpu::new(&program);
    while !cpu.halt {
        cpu.cycle();
    }

    println!("{:02x?}", &cpu.memory[128..128 + 16]);
}
