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
    emit_int,
    emit_iret,
    emit_hlt,
    assemble_source,
    resolve_patches,
};

fn main() {
    let memory_size = 1024;

    let mut memory = Memory::new(memory_size);
    let mut cpu = CPU::new(memory_size as u32);

    let mut pos: u32 = 0;

    // true  → C風コンパイラのテスト
    // false → CPU命令テスト
    let use_compiler_test = false;
    let use_assembler_text_test = true;

    if use_compiler_test {
        compiler::simple_c::compile_simple_c_program(&mut memory, &mut pos);
    } else if use_assembler_text_text {
            let source = "
        LOADI R0, 10
        LOADI R1, 1
        ADD R0, R1
        HLT
        ";
            assemble_source(&mut memory, &mut pos, source);
    } else {
        
        let use_carry_test = true;

        let use_normal_cpu_test = false;

        if use_carry_test {

            emit_loadi(&mut memory, &mut pos, 0, 0);
            emit_loadi(&mut memory, &mut pos, 1, 1);

            emit_sub(&mut memory, &mut pos, 0, 1);

            emit_hlt(&mut memory, &mut pos);
        }

        if use_normal_cpu_test {
            // =========================
            // 今までのCPU命令テスト
            // =========================

            let mut labels = LabelTable::new();
            let mut patches: Vec<Patch> = Vec::new();

            // =========================
            // プログラム
            // =========================

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

            emit_loadi(&mut memory, &mut pos, 0, 111);
            emit_loadi(&mut memory, &mut pos, 1, 222);

            emit_push(&mut memory, &mut pos, 0);
            emit_push(&mut memory, &mut pos, 1);

            emit_pop(&mut memory, &mut pos, 2);
            emit_pop(&mut memory, &mut pos, 3);

            // =========================
            // Phase 5 最終スタックテスト
            // =========================

            emit_loadi(&mut memory, &mut pos, 0, 111);
            emit_loadi(&mut memory, &mut pos, 1, 222);

            emit_push(&mut memory, &mut pos, 0);
            emit_push(&mut memory, &mut pos, 1);

            emit_pop(&mut memory, &mut pos, 2);
            emit_pop(&mut memory, &mut pos, 3);

            // =========================
            // CALL / RET テスト
            // =========================

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
    }

    // =========================
    // Phase 7-6 INT / IRET 基本テスト
    // 消さずに残すが、今は実行しない
    // =========================

    let run_old_interrupt_test = false;

    if run_old_interrupt_test {
        emit_loadi(&mut memory, &mut pos, 0, 10);

        // handler は INT の後の LOADI と HLT を飛ばした先に置く
        let handler_address = pos + 1 + 6 + 1;

        // interrupt_register に割り込み処理の場所を登録
        cpu.interrupt_register = handler_address;

        // 割り込み発生
        emit_int(&mut memory, &mut pos);

        // IRETで戻ってきた後に実行される
        emit_loadi(&mut memory, &mut pos, 7, 777);

        // 戻ってきた後、ここで停止
        emit_hlt(&mut memory, &mut pos);

        // interrupt_handler:
        emit_loadi(&mut memory, &mut pos, 1, 999);

        // 割り込みから戻る
        emit_iret(&mut memory, &mut pos);
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