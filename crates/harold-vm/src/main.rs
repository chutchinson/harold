struct Cpu {
    memory: [u8; 100],
    rip: usize, // rip (register instruction pointer)
    rsp: usize, // rsp (register stack pointer),
    rx: u16,
    ry: u16,
    rz: u16,
    rv: u16,
    halt: bool
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
            halt: false
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
            },
            0x04 => {
                // MOV rx -> rz
                self.rz = self.rx;
                self.rip += 2;
            },
            0x05 => {
                // MOV ry -> rz
                self.rz = self.ry;
                self.rip += 2;
            },
            0x06 => {
                // MOV rx, value
                self.rx = data as u16;
                self.rip += 2;
            },
            0x07 => {
                // MOV ry, value
                self.ry = data as u16;
                self.rip += 2;
            },
            0x08 => {
                // JMP addr
                self.rip = data as usize;
            },
            0x09 => {
                // JZ addr
                if self.rz == 0 {
                    self.rip = data as usize;
                } else {
                    self.rip += 2;
                }
            },
            0x0a => {
                // JG addr
                if self.rz > 0 {
                    self.rip = data as usize;
                } else {
                    self.rip += 2;      
                }
            },
            0x0b => {
                // MOV ry -> rx
                self.rx = self.ry;
                self.rip += 2;
            },
            0x0c => {
                // MOV rz -> ry
                self.ry = self.rz;
                self.rip += 2;
            },
            0x0d => {
                // MOV rv -> rz
                self.rz = self.rv;
                self.rip += 2;
            },
            0x0e => {
                // MOV rv, value
                self.rv = data as u16;
                self.rip += 2;
            },
            0x0f => {
                // DEC rv -> rz
                self.rv = self.rv.wrapping_sub(1);
                self.rz = self.rv;
                self.rip += 2;
            },
            0x10 => {
                // HALT
                self.halt = true;
            },
            _ => {
                // Unknown instruction
                println!("Unknown instruction: {}", opcode);
                self.rip += 2;
            }
        }
    }
}

fn main() {
    // Fibonacci program:
    //   rx = a, ry = b, rv = loop counter
    //
    //   0: SET rx, 0          (06 00)
    //   2: SET ry, 1          (07 01)
    //   4: SET rv, 10         (0e 0a)
    // loop:
    //   6: MOV rx -> rz       (04 00)
    //   8: PRINT rz           (03 00)
    //  10: ADD                (01 00)   rz = rx + ry
    //  12: MOV ry -> rx       (0b 00)   a = b
    //  14: MOV rz -> ry       (0c 00)   b = next
    //  16: DEC rv -> rz       (0f 00)   rv -= 1, rz = rv
    //  18: JG 6               (0a 06)   loop while rz > 0
    //  20: HALT               (10 00)
    let program: [u8; 22] = [
        0x06, 0x00,
        0x07, 0x01,
        0x0e, 0x0a,
        0x04, 0x00,
        0x03, 0x00,
        0x01, 0x00,
        0x0b, 0x00,
        0x0c, 0x00,
        0x0f, 0x00,
        0x0a, 0x06,
        0x10, 0x00,
    ];

    // char* data = malloc(sizeof(char) * 100);

    let mut cpu = Cpu::new(&program);
    while !cpu.halt {
        cpu.cycle();
    }
}
