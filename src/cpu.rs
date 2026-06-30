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
pub const MOV: u8 = 9;     // レジスタ間で値をコピーする
pub const HLT: u8 = 255;   // 停止

pub struct CPU {
    pub registers: [u32; 8], // R0〜R7
    pub pc: u32,             // Program Counter
    pub ir: u8,              // Instruction Register
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0; 8],
            pc: 0,
            ir: 0,
        }
    }

    // CPUの状態を表示する
    pub fn dump_registers(&self) {
        println!("PC = {}", self.pc);
        println!("IR = {}", self.ir);

        for i in 0..8 {
            println!("R{} = {}", i, self.registers[i]);
        }

        println!("--------------------");
    }

    // 1byte読む
    fn fetch_u8(&mut self, memory: &Memory) -> u8 {
        let value = memory.read_u8(self.pc);
        self.pc = self.pc.wrapping_add(1);
        value
    }

    // 4byte読む
    fn fetch_u32(&mut self, memory: &Memory) -> u32 {
        let value = memory.read_u32(self.pc);
        self.pc = self.pc.wrapping_add(4);
        value
    }

    // レジスタ番号が正しいか確認する
    fn check_register(reg: u8) -> usize {
        if reg >= 8 {
            panic!("Invalid register: R{}", reg);
        }

        reg as usize
    }

    // 命令を1つ実行する
    // true  → 停止
    // false → 続行
    pub fn step(&mut self, memory: &mut Memory) -> bool {
        // 命令番号を読む
        self.ir = self.fetch_u8(memory);

        match self.ir {
            LOAD => {
                // LOAD R0, 100
                // [LOAD][reg][address u32]
                let reg = self.fetch_u8(memory);
                let address = self.fetch_u32(memory);

                let reg_index = CPU::check_register(reg);
                self.registers[reg_index] = memory.read_u32(address);
            }

            LOADI => {
                // LOADI R0, 10
                // [LOADI][reg][value u32]
                let reg = self.fetch_u8(memory);
                let value = self.fetch_u32(memory);

                let reg_index = CPU::check_register(reg);
                self.registers[reg_index] = value;
            }

            STORE => {
                // STORE R0, 100
                // [STORE][reg][address u32]
                let reg = self.fetch_u8(memory);
                let address = self.fetch_u32(memory);

                let reg_index = CPU::check_register(reg);
                memory.write_u32(address, self.registers[reg_index]);
            }

            MOV => {
                // MOV R3, R0
                // R3 = R0
                // [MOV][dst][src]
                let dst = self.fetch_u8(memory);
                let src = self.fetch_u8(memory);

                let dst_index = CPU::check_register(dst);
                let src_index = CPU::check_register(src);

                self.registers[dst_index] = self.registers[src_index];
            }

            ADD => {
                // ADD R3, R1
                // R3 = R3 + R1
                // [ADD][dst][src]
                let dst = self.fetch_u8(memory);
                let src = self.fetch_u8(memory);

                let dst_index = CPU::check_register(dst);
                let src_index = CPU::check_register(src);

                self.registers[dst_index] =
                    self.registers[dst_index].wrapping_add(self.registers[src_index]);
            }

            SUB => {
                // SUB R3, R2
                // R3 = R3 - R2
                // [SUB][dst][src]
                let dst = self.fetch_u8(memory);
                let src = self.fetch_u8(memory);

                let dst_index = CPU::check_register(dst);
                let src_index = CPU::check_register(src);

                self.registers[dst_index] =
                    self.registers[dst_index].wrapping_sub(self.registers[src_index]);
            }

            JMP => {
                // JMP 30
                // [JMP][address u32]
                let address = self.fetch_u32(memory);
                self.pc = address;
            }

            JZ => {
                // JZ R0, 30
                // R0が0なら30番地へジャンプ
                // [JZ][reg][address u32]
                let reg = self.fetch_u8(memory);
                let address = self.fetch_u32(memory);

                let reg_index = CPU::check_register(reg);

                if self.registers[reg_index] == 0 {
                    self.pc = address;
                }
            }

            JNZ => {
                // JNZ R0, 12
                // R0が0でなければ12番地へジャンプ
                // [JNZ][reg][address u32]
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