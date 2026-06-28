use crate::memory::Memory;

// 命令番号
pub const LOAD: u8 = 1;    // メモリからレジスタへ読み込む
pub const STORE: u8 = 2;   // レジスタからメモリへ保存する
pub const ADD: u8 = 3;     // レジスタ同士の足し算
pub const SUB: u8 = 4;     // レジスタ同士の引き算
pub const JMP: u8 = 5;     // 無条件ジャンプ
pub const JZ: u8 = 6;      // レジスタが0ならジャンプ
pub const JNZ: u8 = 7;     // レジスタが0でなければジャンプ
pub const LOADI: u8 = 8;   // 即値をレジスタへ入れる
pub const HLT: u8 = 255;   // 停止

pub struct CPU {
    pub registers: [u32; 8], // R0〜R7
    pub pc: u32,
    pub ir: u8,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0; 8],
            pc: 0,
            ir: 0,
        }
    }

    fn fetch_u8(&mut self, memory: &Memory) -> u8 {
        let value = memory.read_u8(self.pc);
        self.pc = self.pc.wrapping_add(1);
        value
    }

    fn fetch_u32(&mut self, memory: &Memory) -> u32 {
        let value = memory.read_u32(self.pc);
        self.pc = self.pc.wrapping_add(4);
        value
    }

    fn check_register(reg: u8) -> usize {
        if reg >= 8 {
            panic!("Invalid register: R{}", reg);
        }

        reg as usize
    }

    pub fn step(&mut self, memory: &mut Memory) -> bool {
        self.ir = self.fetch_u8(memory);

        match self.ir {
            LOAD => {
                let reg = self.fetch_u8(memory);
                let address = self.fetch_u32(memory);

                let reg_index = CPU::check_register(reg);
                self.registers[reg_index] = memory.read_u32(address);
            }

            LOADI => {
                // LOADI R0, 3
                // [LOADI][reg][value u32]
                let reg = self.fetch_u8(memory);
                let value = self.fetch_u32(memory);

                let reg_index = CPU::check_register(reg);
                self.registers[reg_index] = value;
            }

            STORE => {
                let reg = self.fetch_u8(memory);
                let address = self.fetch_u32(memory);

                let reg_index = CPU::check_register(reg);
                memory.write_u32(address, self.registers[reg_index]);
            }

            ADD => {
                let dst = self.fetch_u8(memory);
                let src = self.fetch_u8(memory);

                let dst_index = CPU::check_register(dst);
                let src_index = CPU::check_register(src);

                self.registers[dst_index] =
                    self.registers[dst_index].wrapping_add(self.registers[src_index]);
            }

            SUB => {
                let dst = self.fetch_u8(memory);
                let src = self.fetch_u8(memory);

                let dst_index = CPU::check_register(dst);
                let src_index = CPU::check_register(src);

                self.registers[dst_index] =
                    self.registers[dst_index].wrapping_sub(self.registers[src_index]);
            }

            JMP => {
                let address = self.fetch_u32(memory);
                self.pc = address;
            }

            JZ => {
                let reg = self.fetch_u8(memory);
                let address = self.fetch_u32(memory);

                let reg_index = CPU::check_register(reg);

                if self.registers[reg_index] == 0 {
                    self.pc = address;
                }
            }

            JNZ => {
                let reg = self.fetch_u8(memory);
                let address = self.fetch_u32(memory);

                let reg_index = CPU::check_register(reg);

                if self.registers[reg_index] != 0 {
                    self.pc = address;
                }
            }

            HLT => {
                return true;
            }

            _ => {
                println!("Unknown instruction: {}", self.ir);
                return true;
            }
        }

        false
    }
}