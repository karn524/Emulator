mod cpu;
mod memory;

use cpu::{CPU, LOADI, STORE, SUB, JNZ, HLT};
use memory::Memory;

fn main() {
    let mut memory = Memory::new(1024);
    let mut cpu = CPU::new();

    // =========================
    // プログラム
    // =========================
    // 0:  LOADI R0, 3      R0 = 3
    // 6:  LOADI R1, 1      R1 = 1
    // 12: SUB R0, R1       R0 = R0 - R1
    // 15: JNZ R0, 12       R0が0でなければ12番地へ戻る
    // 21: STORE R0, 100    memory[100] = R0
    // 27: HLT

    // LOADI R0, 3
    memory.write_u8(0, LOADI);
    memory.write_u8(1, 0);       // R0
    memory.write_u32(2, 3);      // value 3

    // LOADI R1, 1
    memory.write_u8(6, LOADI);
    memory.write_u8(7, 1);       // R1
    memory.write_u32(8, 1);      // value 1

    // SUB R0, R1
    memory.write_u8(12, SUB);
    memory.write_u8(13, 0);      // R0
    memory.write_u8(14, 1);      // R1

    // JNZ R0, 12
    memory.write_u8(15, JNZ);
    memory.write_u8(16, 0);      // R0
    memory.write_u32(17, 12);    // jump address

    // STORE R0, 100
    memory.write_u8(21, STORE);
    memory.write_u8(22, 0);      // R0
    memory.write_u32(23, 100);   // address 100

    // HLT
    memory.write_u8(27, HLT);

    loop {
        let stop = cpu.step(&mut memory);

        if stop {
            break;
        }
    }

    println!("R0 = {}", cpu.registers[0]);
    println!("R1 = {}", cpu.registers[1]);
    println!("memory[100] = {}", memory.read_u32(100));
}