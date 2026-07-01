mod cpu;
mod memory;
mod assembler;

use cpu::CPU;
use memory::Memory;
use assembler::{
    LabelTable,
    emit_loadi,
    emit_sub,
    emit_store,
    emit_jmp,
    emit_jz,
    emit_hlt,
    patch_u32,
};

fn main() {
    let mut memory = Memory::new(1024);
    let mut cpu = CPU::new();

    let mut pos: u32 = 0;
    let mut labels = LabelTable::new();

    // =========================
    // プログラム
    // =========================
    // LOADI R0, 3
    // LOADI R1, 1
    //
    // loop:
    // JZ R0, end
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

    // JZ R0, end
    // この時点では end の番地がまだ分からないので、仮に 0 を入れる
    let jz_address_pos = pos + 2;
    emit_jz(&mut memory, &mut pos, 0, 0);

    // SUB R0, R1
    emit_sub(&mut memory, &mut pos, 0, 1);

    // JMP loop
    let loop_address = labels.get("loop");
    emit_jmp(&mut memory, &mut pos, loop_address);

    // end:
    labels.define("end", pos);

    // さっき仮に 0 にしていた JZ のジャンプ先を end の番地に書き換える
    let end_address = labels.get("end");
    patch_u32(&mut memory, jz_address_pos, end_address);

    // STORE R0, 100
    emit_store(&mut memory, &mut pos, 0, 100);

    // HLT
    emit_hlt(&mut memory, &mut pos);

    // =========================
    // CPU実行ループ
    // =========================
    loop {
        cpu.dump_registers();

        let stop = cpu.step(&mut memory);

        if stop {
            break;
        }
    }

    cpu.dump_registers();

    println!("memory[100] = {}", memory.read_u32(100));
}