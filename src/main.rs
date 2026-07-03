mod emulator;
mod assembler;

use emulator::cpu::CPU;
use emulator::memory::Memory;

use assembler::emit::{
    LabelTable,
    Patch,
    emit_loadi,
    emit_sub,
    emit_store,
    emit_jz_label,
    emit_jmp_label,
    emit_hlt,
    resolve_patches,
};

fn main() {
    let mut memory = Memory::new(1024);
    let mut cpu = CPU::new();

    let mut pos: u32 = 0;
    let mut labels = LabelTable::new();
    let mut patches: Vec<Patch> = Vec::new();

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
    emit_jz_label(&mut memory, &mut pos, 0, "end", &mut patches);

    // SUB R0, R1
    emit_sub(&mut memory, &mut pos, 0, 1);

    // JMP loop
    emit_jmp_label(&mut memory, &mut pos, "loop", &mut patches);

    // end:
    labels.define("end", pos);

    // さっき仮に 0 にしていた JZ のジャンプ先を end の番地に書き換える
    resolve_patches(&mut memory, &labels, &patches);

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