mod cpu;
mod memory;
mod assembler;

use cpu::CPU;
use memory::Memory;
use assembler::{
    emit_loadi,
    emit_mov,
    emit_add,
    emit_sub,
    emit_store,
    emit_hlt,
};

fn main() {
    let mut memory = Memory::new(1024);
    let mut cpu = CPU::new();

    let mut pos: u32 = 0;

    // =========================
    // プログラム
    // =========================
    // LOADI R0, 10
    // LOADI R1, 3
    // LOADI R2, 2
    // MOV R3, R0
    // ADD R3, R1
    // SUB R3, R2
    // STORE R3, 100
    // HLT

    emit_loadi(&mut memory, &mut pos, 0, 10);
    emit_loadi(&mut memory, &mut pos, 1, 3);
    emit_loadi(&mut memory, &mut pos, 2, 2);

    emit_mov(&mut memory, &mut pos, 3, 0);
    emit_add(&mut memory, &mut pos, 3, 1);
    emit_sub(&mut memory, &mut pos, 3, 2);

    emit_store(&mut memory, &mut pos, 3, 100);
    emit_hlt(&mut memory, &mut pos);

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