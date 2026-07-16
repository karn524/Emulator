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
pub const PUSH: u8 = 12;   // レジスタの値をスタックに積む
pub const POP: u8 = 13;    // スタックから値を取り出してレジスタに入れる
pub const CALL: u8 = 14;   // 関数呼び出し
pub const RET: u8 = 15;    // 関数から戻る
pub const INT: u8 = 16;    // 割り込み処理へ移動する
pub const IRET: u8 = 17;   // 割り込み処理から戻る
pub const ENTER: u8 = 18;  // 
pub const LEAVE: u8 = 19;  //
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

    // スタックの状態を表示する
    pub fn dump_stack(&self, memory: &Memory) {
        println!("===== STACK =====");
        println!("SP = {}", self.sp);

        let memory_size = memory.data.len() as u32;

        if self.sp >= memory_size {
            println!("stack is empty");
        } else {
            let mut address = self.sp;

            while address < memory_size {
                let value = memory.read_u32(address);
                println!("memory[{}] = {}", address, value);
                address += 4;
            }
        }

        println!("=================");
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

                let a = self.registers[dst_index];
                let b = self.registers[src_index];

                let result = a.wrapping_add(b);

                self.carry_flag = result < a;

                self.registers[dst_index] = result;

                self.update_zero_sign_flags(self.registers[dst_index]);

                // overflow_flagはPhase 12で実装予定
            }

            SUB => {
                // SUB R3, R2
                // R3 = R3 - R2
                // [SUB][dst][src]
                let dst = self.fetch_u8(memory);
                let src = self.fetch_u8(memory);

                let dst_index = CPU::check_register(dst);
                let src_index = CPU::check_register(src);

                let a = self.registers[dst_index];
                let b = self.registers[src_index];

                let result = a.wrapping_sub(b);

                // 符号なし減算で借りが発生した場合、carry_flag = true
                // 例: 0 - 1 は u32 では 4294967295 になるので、a < b になる
                self.carry_flag = a < b;

                self.registers[dst_index] = result;

                self.update_zero_sign_flags(self.registers[dst_index]);

                // overflow_flag は Phase 12 で実装予定
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

            PUSH => {
                // PUSH R0
                // R0 の値をスタックに積む
                // [PUSH][reg]
                let reg = self.fetch_u8(memory);

                let reg_index = CPU::check_register(reg);

                //スタックがこれ以上に伸びられないならエラー
                if self.sp < 4 {
                    panic!("Stack overflow: PUSH exceed stak area");
                }

                // スタックは4byte単位で下に伸びる
                self.sp = self.sp.wrapping_sub(4);

                let value = self.registers[reg_index];
                memory.write_u32(self.sp, value);

                println!(
                    "PUSH R{} value={} to memory[{}]",
                    reg,
                    value,
                    self.sp
                );
            }

            POP => {
                // POP R1
                // スタックから値を取り出して R1 に入れる
                // [POP][reg]
                let reg = self.fetch_u8(memory);

                let reg_index = CPU::check_register(reg);

                // スタックが空ならエラー
                if self.sp as usize >= memory.data.len() {
                    panic!("Stack underflow: POP from empty stack");
                }

                let value = memory.read_u32(self.sp);
                self.registers[reg_index] = value;

                println!(
                    "POP R{} value={} from memory[{}]",
                    reg,
                    value,
                    self.sp
                );

                // 取り出したのでSPを戻す
                self.sp = self.sp.wrapping_add(4);

                self.update_zero_sign_flags(self.registers[reg_index]);
            }

            CALL => {
                // CALL address
                // 現在のPCを戻り先としてスタックに積み、
                // address にジャンプする
                // [CALL][address u32]

                let address = self.fetch_u32(memory);

                // CALL命令を読み終わった時点のPCが戻り先
                let return_address = self.pc;

                // スタックがこれ以上下に伸びられないならエラー
                if self.sp < 4 {
                    panic!("Stack overflow: CALL cannot push return address");
                }

                // 戻り先アドレスをスタックに積む
                self.sp = self.sp.wrapping_sub(4);
                memory.write_u32(self.sp, return_address);

                    println!(
                        "CALL address={} return_address={} pushed to memory[{}]",
                        address,
                        return_address,
                        self.sp
                        );

                // 関数の場所へジャンプ
                self.pc = address;
            }

            RET => {
                // RET
                // スタックから戻り先アドレスを取り出して戻る
                // [RET]

                // スタックが空ならエラー
                if self.sp as usize >= memory.data.len() {
                    panic!("Stack underflow: RET without return address");
                }

                let return_address = memory.read_u32(self.sp);

                println!(
                    "RET return_address={} from memory[{}]",
                    return_address,
                    self.sp
                );

                self.sp = self.sp.wrapping_add(4);

                self.pc = return_address;
            }

            INT => {
                // INT
                // interrupt_register に設定された割り込み処理へジャンプする
                // [INT]

                let return_address = self.pc;

                if self.interrupt_register == 0 {
                    panic!("Interrupt handler is not set");
                }

                if self.sp < 4 {
                    panic!("Stack overflow: INT cannot push return address");
                }

                self.sp = self.sp.wrapping_sub(4);
                memory.write_u32(self.sp, return_address);

                println!(
                    "INT handler={} return_address={} pushed to memory[{}]",
                    self.interrupt_register,
                    return_address,
                    self.sp
                );

                self.pc = self.interrupt_register;
            }

            IRET => {
                // IRET
                // 割り込み処理から戻る
                // [IRET]

                if self.sp as usize >= memory.data.len() {
                    panic!("Stack underflow: IRET without return address");
                }

                let return_address = memory.read_u32(self.sp);

                println!(
                    "IRET return_address={} from memory[{}]",
                    return_address,
                    self.sp
                );

                self.sp = self.sp.wrapping_add(4);

                self.pc = return_address;
            }
            ENTER => {
                // 古いBPをスタックに保存する
                if self.sp < 4 {
                    panic!("Stack overflow on ENTER");
                }

                self.sp -= 4;
                memory.write_u32(self.sp, self.bp);

                // 現在のSPを新しいBPにする
                self.bp = self.sp;

                // ローカル変数用に8バイト確保する
                if self.sp < 8 {
                    panic!("Stack overflow on ENTER local area");
                }

                self.sp -= 8;
            }

            LEAVE => {
                // SPを現在のBPの位置に戻す
                self.sp = self.bp;

                // 古いBPをスタックから復元する
                self.bp = memory.read_u32(self.sp);
                self.sp += 4;
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