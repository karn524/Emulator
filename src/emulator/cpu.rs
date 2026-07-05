use crate::emulator::memory::Memory;

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
pub const JS: u8 = 10;     // sign_flag が true ならジャンプ
pub const JNS: u8 = 11;    // sign_flag が false ならジャンプ
pub const HLT: u8 = 255;   // 停止

pub struct CPU {
    // 汎用レジスタ
    // R0〜R7：計算・一時保存・メモリ読み書きに使う
    pub registers: [u32; 8],

    // 命令実行用レジスタ
    // pc：次に読む命令のアドレス
    // ir：現在実行中の命令番号
    pub pc: u32,
    pub ir: u8,

    // 割り込み用レジスタ
    // Phase 7で使用予定
    pub interrupt_register: u32,

    // スタック・関数呼び出し用レジスタ
    // sp：スタックの現在位置
    // bp：ベースポインタ
    // fp：フレームポインタ
    // lr：関数呼び出し後に戻るアドレス
    pub sp: u32,
    pub bp: u32,
    pub fp: u32,
    pub lr: u32,

    // フラグレジスタ
    // Phase 3で本格的に更新処理を入れる
    pub carry_flag: bool,
    pub sign_flag: bool,
    pub zero_flag: bool,
    pub overflow_flag: bool,
}

impl CPU {
    pub fn new(memory_size: u32) -> Self {
        CPU {
            registers: [0; 8],

            pc: 0,
            ir: 0,

            interrupt_register: 0,

            sp: memory_size,
            bp: memory_size,
            fp: memory_size,
            lr: 0,

            carry_flag: false,
            sign_flag: false,
            zero_flag: false,
            overflow_flag: false,
        }
    }

    // CPUの状態を表示する
    pub fn dump_registers(&self) {
        println!("PC = {}", self.pc);
        println!("IR = {}", self.ir);
        println!("INT = {}", self.interrupt_register);

        for i in 0..8 {
        println!("R{} = {}", i, self.registers[i]);
        }

        println!("SP = {}", self.sp);
        println!("BP = {}", self.bp);
        println!("FP = {}", self.fp);
        println!("LR = {}", self.lr);

        println!("CF = {}", self.carry_flag);
        println!("SF = {}", self.sign_flag);
        println!("ZF = {}", self.zero_flag);
        println!("OF = {}", self.overflow_flag);

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

    fn update_zero_sign_flags(&mut self, value: u32) {
        self.zero_flag = value == 0;
        self.sign_flag = (value & 0x8000_0000) != 0;
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
                self.update_zero_sign_flags(self.registers[reg_index]);
            }

            LOADI => {
                // LOADI R0, 10
                // [LOADI][reg][value u32]
                let reg = self.fetch_u8(memory);
                let value = self.fetch_u32(memory);

                let reg_index = CPU::check_register(reg);
                self.registers[reg_index] = value;
                self.update_zero_sign_flags(self.registers[reg_index]);
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
                self.update_zero_sign_flags(self.registers[dst_index]);
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

                self.update_zero_sign_flags(self.registers[dst_index]);
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

                self.update_zero_sign_flags(self.registers[dst_index]);
            }

            JMP => {
                // JMP 30
                // [JMP][address u32]
                let address = self.fetch_u32(memory);
                self.pc = address;
            }

            JZ => {
                // JZ adress
                // zero_flag が true ならジャンプ
                // [JZ][address u32]
                let address = self.fetch_u32(memory);

                if self.zero_flag {
                    self.pc = address;
                }
            }

            JNZ => {
                // JNZ adress
                // zero_flagがfalseならジャンプ
                // [JNZ][address u32]
                let address = self.fetch_u32(memory);

                if !self.zero_flag {
                    self.pc = address;
                }
            }

            JS => {
                // JS address
                // sign_flag が true ならジャンプ
                // [JS][address u32]
                let address = self.fetch_u32(memory);

                if self.sign_flag {
                    self.pc = address;
                }
            }
        

            JNS => {
                // JNS address
                // sign_flag が false ならジャンプ
                // [JNS][address u32]
                let address = self.fetch_u32(memory);

                if !self.sign_flag {
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