mod emulator;
mod assembler;
mod compiler;

use emulator::cpu::CPU;
use emulator::memory::Memory;

use assembler::emit::{
    LabelTable,
    Patch,
    emit_load,
    emit_loadi,
    emit_mov,
    emit_add,
    emit_sub,
    emit_store,
    emit_jz_label,
    emit_jmp_label,
    emit_jnz,
    emit_js,
    emit_jns,
    emit_push,
    emit_pop,
    emit_call,
    emit_ret,
    emit_hlt,
    resolve_patches,
};

fn main() {
    let memory_size = 1024;
    
    let mut memory = Memory::new(memory_size);
    let mut cpu = CPU::new(memory_size as u32);

    let mut pos: u32 = 0;

    // true  → C風コンパイラのテスト
    // false → 今までのCPU命令テスト
    let use_compiler_test = false;

    if use_compiler_test {

        // =========================
        // C風ミニコンパイラのテスト
        // =========================

        compiler::simple_c::compile_simple_c_program(&mut memory, &mut pos);
    } else {

        // =========================
        // 今までのCPU命令テスト
        // =========================

        let mut labels = LabelTable::new();
        let mut patches: Vec<Patch> = Vec::new();

    // =========================
    // プログラム
    // =========================
    // LOADI R0, 3
    // LOADI R1, 1
    //
    // loop:
    // JZ end           //zero_flagがtrueならendへ
    // SUB R0, R1
    // JMP loop
    //
    // end:
    // STORE R0, 100
    // HLT

    // LOADI R0, 3
    emit_loadi(&mut memory, &mut pos, 0, 3);

    // LOADI R1, 1
    emit_loadi(&mut memory, &mut pos, 1, 1);

    // loop:
    labels.define("loop", pos);

    // JZ end
    emit_jz_label(&mut memory, &mut pos, "end", &mut patches);

    // SUB R0, R1
    emit_sub(&mut memory, &mut pos, 0, 1);

    // JMP loop
    emit_jmp_label(&mut memory, &mut pos, "loop", &mut patches);

    // end:
    labels.define("end", pos);

    // STORE R0, 400
    // ループが終わったので、R0は0になっているはず
    emit_store(&mut memory, &mut pos, 0, 400);

    // LOAD R2, 400
    // memory[400] の値を R2 に読み込む
    emit_load(&mut memory, &mut pos, 2, 400);

    // LOADI R3, 10
    emit_loadi(&mut memory, &mut pos, 3, 10);

    // MOV R4, R3
    // R4 = R3 = 10
    emit_mov(&mut memory, &mut pos, 4, 3);

    // ADD R4, R3
    // R4 = 10 + 10 = 20
    emit_add(&mut memory, &mut pos, 4, 3);

    // STORE R4, 400
    // memory[400] = 20
    emit_store(&mut memory, &mut pos, 4, 400);

    // さっき仮に 0 にしていたジャンプ先を本当の番地に書き換える
    resolve_patches(&mut memory, &labels, &patches);

    // =========================
    // JNZ テスト
    // =========================
    // R5 = 1
    // JNZ R5, skip
    // LOADI R6, 999   ← ここは実行されないはず
    // skip:
    // LOADI R7, 777   ← ここは実行されるはず

    emit_loadi(&mut memory, &mut pos, 5, 1);

    // LOADI R5, 1 によって zero_flag は false になる
    // JNZ は zero_flag == false ならジャンプする
    let skip_address = pos + 5 + 6;

    // JNZ 自体は 5 byte
    // LOADI R6, 999 は 6 byte
    emit_jnz(&mut memory, &mut pos, skip_address);

    // これは飛ばされるはず
    emit_loadi(&mut memory, &mut pos, 6, 999);

    // skip:
    emit_loadi(&mut memory, &mut pos, 7, 777);

    // =========================
    // JS テスト
    // =========================
    // R0 = 0
    // R1 = 1
    // SUB R0, R1  → 0 - 1 なので sign_flag = true
    // JS minus
    // LOADI R6, 1234   ← ここは飛ばされるはず
    // minus:
    // LOADI R6, 5555   ← ここは実行されるはず

    emit_loadi(&mut memory, &mut pos, 0, 0);
    emit_loadi(&mut memory, &mut pos, 1, 1);
    emit_sub(&mut memory, &mut pos, 0, 1);

    // JS は 5 byte
    // LOADI R6, 1234 は 6 byte
    let minus_address = pos + 5 + 6;

    emit_js(&mut memory, &mut pos, minus_address);

    // これは飛ばされるはず
    emit_loadi(&mut memory, &mut pos, 6, 1234);

    // minus:
    emit_loadi(&mut memory, &mut pos, 6, 5555);

    // =========================
    // JNS テスト
    // =========================
    // R0 = 5
    // R1 = 1
    // SUB R0, R1  → 5 - 1 = 4 なので sign_flag = false
    // JNS plus
    // LOADI R7, 1234   ← ここは飛ばされるはず
    // plus:
    // LOADI R7, 8888   ← ここは実行されるはず

    emit_loadi(&mut memory, &mut pos, 0, 5);
    emit_loadi(&mut memory, &mut pos, 1, 1);
    emit_sub(&mut memory, &mut pos, 0, 1);

    // JNS は 5 byte
    // LOADI R7, 1234 は 6 byte
    let plus_address = pos + 5 + 6;

    emit_jns(&mut memory, &mut pos, plus_address);

    // これは飛ばされるはず
    emit_loadi(&mut memory, &mut pos, 7, 1234);

    // plus:
    emit_loadi(&mut memory, &mut pos, 7, 8888);

    // =========================
    // PUSH / POP テスト
    // =========================
    // R0 = 123
    // PUSH R0
    // R0 = 0
    // POP R1
    //
    // 期待結果:
    // R1 = 123
    // SP = 1024 に戻る

    emit_loadi(&mut memory, &mut pos, 0, 123);

    // R0 の値をスタックに積む
    emit_push(&mut memory, &mut pos, 0);

    // R0 を 0 にして、ちゃんとスタックから戻せているか確認する
    emit_loadi(&mut memory, &mut pos, 0, 0);

    // スタックから取り出して R1 に入れる
    emit_pop(&mut memory, &mut pos, 1);

    // =========================
    // 複数 PUSH / POP テスト
    // =========================
    // スタックは後入れ先出し
    //
    // R0 = 111
    // R1 = 222
    // PUSH R0
    // PUSH R1
    // POP R2  → R2 = 222
    // POP R3  → R3 = 111

    emit_loadi(&mut memory, &mut pos, 0, 111);
    emit_loadi(&mut memory, &mut pos, 1, 222);

    emit_push(&mut memory, &mut pos, 0);
    emit_push(&mut memory, &mut pos, 1);

    emit_pop(&mut memory, &mut pos, 2);
    emit_pop(&mut memory, &mut pos, 3);

    // =========================
    // Phase 5 最終スタックテスト
    // =========================
    // R0 = 111
    // R1 = 222
    // PUSH R0
    // PUSH R1
    // POP R2
    // POP R3
    //
    // 期待結果:
    // R2 = 222
    // R3 = 111
    // SP = 1024

    emit_loadi(&mut memory, &mut pos, 0, 111);
    emit_loadi(&mut memory, &mut pos, 1, 222);

    emit_push(&mut memory, &mut pos, 0);
    emit_push(&mut memory, &mut pos, 1);

    emit_pop(&mut memory, &mut pos, 2);
    emit_pop(&mut memory, &mut pos, 3); 

    // =========================
    // CALL / RET テスト
    // =========================
    // CALL func
    // LOADI R7, 777
    // HLT
    //
    // func:
    // LOADI R0, 123
    // RET

    // CALL は 5 byte
    // LOADI R7, 777 は 6 byte
    // HLT は 1 byte
    //
    // func は CALL の後の LOADI と HLT を飛ばした先に置く
    let func_address = pos + 5 + 6 + 1;

    // CALL func
    emit_call(&mut memory, &mut pos, func_address);

    // RETで戻ってきた後に実行される
    emit_loadi(&mut memory, &mut pos, 7, 777);

    // ここで一度止まる
    emit_hlt(&mut memory, &mut pos);

    // func:
    emit_loadi(&mut memory, &mut pos, 0, 123);

    // 関数から戻る
    emit_ret(&mut memory, &mut pos);

    // HLT
    emit_hlt(&mut memory, &mut pos);
    }

    // =========================
    // CPU実行ループ
    // =========================
    loop {
        cpu.dump_registers();
        cpu.dump_stack(&memory);

        let stop = cpu.step(&mut memory);

        if stop {
            break;
        }
    }

    cpu.dump_registers();

    println!("memory[400] = {}", memory.read_u32(400));
    println!("memory[404] = {}", memory.read_u32(404));
}