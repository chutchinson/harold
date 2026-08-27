struct Cpu {
    memory: [u8; 100],
    rip: usize, // rip (register instruction pointer)
    rsp: usize, // rsp (register stack pointer),
    rx: u16,
    ry: u16,
    rz: u16,
    rv: u16,
    halt: bool,
}

impl Cpu {
    pub fn new(program: &[u8]) -> Self {
        let mut memory = [0; 100];
        memory[..program.len()].copy_from_slice(program);

        Cpu {
            memory,
            rip: 0,
            rsp: 0,
            rx: 0,
            ry: 0,
            rz: 0,
            rv: 0,
            halt: false,
        }
    }

    pub fn cycle(&mut self) {
        // fetch-decode-execute
        if self.rip >= self.memory.len() {
            self.halt = true;
            return;
        }

        // fetch
        let opcode = self.memory[self.rip];
        let data = self.memory[self.rip + 1];

        // decode
        match opcode {
            0x00 => {
                // NOP
                self.rip += 2;
            }
            0x01 => {
                // ADD rx, ry -> rz
                self.rz = self.rx + self.ry;
                self.rip += 2;
            }
            0x02 => {
                // SUB rx, ry -> rz
                self.rz = self.rx - self.ry;
                self.rip += 2;
            }
            0x03 => {
                // PRINT rz
                println!("{}", self.rz);
                self.rip += 2;
            }
            0x04 => {
                // MOV rx -> rz
                self.rz = self.rx;
                self.rip += 2;
            }
            0x05 => {
                // MOV ry -> rz
                self.rz = self.ry;
                self.rip += 2;
            }
            0x06 => {
                // MOV rx, value
                self.rx = data as u16;
                self.rip += 2;
            }
            0x07 => {
                // MOV ry, value
                self.ry = data as u16;
                self.rip += 2;
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
                    self.rip += 2;
                }
            }
            0x0a => {
                // JG addr
                if self.rz > 0 {
                    self.rip = data as usize;
                } else {
                    self.rip += 2;
                }
            }
            0x0b => {
                // MOV ry -> rx
                self.rx = self.ry;
                self.rip += 2;
            }
            0x0c => {
                // MOV rz -> ry
                self.ry = self.rz;
                self.rip += 2;
            }
            0x0d => {
                // MOV rv -> rz
                self.rz = self.rv;
                self.rip += 2;
            }
            0x0e => {
                // MOV rv, value
                self.rv = data as u16;
                self.rip += 2;
            }
            0x0f => {
                // DEC rv -> rz
                self.rv = self.rv.wrapping_sub(1);
                self.rz = self.rv;
                self.rip += 2;
            }
            0x10 => {
                // HALT
                self.halt = true;
            }
            _ => {
                // Unknown instruction
                println!("Unknown instruction: {}", opcode);
                self.rip += 2;
            }
        }
    }
}

fn main() {
    let program = include_bytes!("../../debug");

    let mut cpu = Cpu::new(program);
    while !cpu.halt {
        cpu.cycle();
    }
}
