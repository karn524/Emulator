use crate::cpu::{LOADI, STORE, ADD, SUB, MOV, HLT};
use crate::memory::Memory;

// LOADI Rn, value
pub fn emit_loadi(memory: &mut Memory, pos: &mut u32, reg: u8, value: u32) {
    memory.write_u8(*pos, LOADI);
    *pos += 1;

    memory.write_u8(*pos, reg);
    *pos += 1;

    memory.write_u32(*pos, value);
    *pos += 4;
}

// MOV dst, src
pub fn emit_mov(memory: &mut Memory, pos: &mut u32, dst: u8, src: u8) {
    memory.write_u8(*pos, MOV);
    *pos += 1;

    memory.write_u8(*pos, dst);
    *pos += 1;

    memory.write_u8(*pos, src);
    *pos += 1;
}

// ADD dst, src
pub fn emit_add(memory: &mut Memory, pos: &mut u32, dst: u8, src: u8) {
    memory.write_u8(*pos, ADD);
    *pos += 1;

    memory.write_u8(*pos, dst);
    *pos += 1;

    memory.write_u8(*pos, src);
    *pos += 1;
}

// SUB dst, src
pub fn emit_sub(memory: &mut Memory, pos: &mut u32, dst: u8, src: u8) {
    memory.write_u8(*pos, SUB);
    *pos += 1;

    memory.write_u8(*pos, dst);
    *pos += 1;

    memory.write_u8(*pos, src);
    *pos += 1;
}

// STORE Rn, address
pub fn emit_store(memory: &mut Memory, pos: &mut u32, reg: u8, address: u32) {
    memory.write_u8(*pos, STORE);
    *pos += 1;

    memory.write_u8(*pos, reg);
    *pos += 1;

    memory.write_u32(*pos, address);
    *pos += 4;
}

// HLT
pub fn emit_hlt(memory: &mut Memory, pos: &mut u32) {
    memory.write_u8(*pos, HLT);
    *pos += 1;
}