use std::collections::HashMap;
use crate::cpu::{LOADI, STORE, ADD, SUB, MOV, JMP, JZ, JNZ, HLT};
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

// JMP address
pub fn emit_jmp(memory: &mut Memory, pos: &mut u32, address: u32) {
    memory.write_u8(*pos, JMP);
    *pos += 1;

    memory.write_u32(*pos, address);
    *pos += 4;
}

// JZ Rn, address
pub fn emit_jz(memory: &mut Memory, pos: &mut u32, reg: u8, address: u32) {
    memory.write_u8(*pos, JZ);
    *pos += 1;

    memory.write_u8(*pos, reg);
    *pos += 1;

    memory.write_u32(*pos, address);
    *pos += 4;
}

// JNZ Rn, address
pub fn emit_jnz(memory: &mut Memory, pos: &mut u32, reg: u8, address: u32) {
    memory.write_u8(*pos, JNZ);
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

// 仮に書いておいた32bit値をあとから書き換える
pub fn patch_u32(memory: &mut Memory, address_pos: u32, value: u32) {
    memory.write_u32(address_pos, value);
}

pub struct LabelTable {
    labels: HashMap<String, u32>,
}

impl LabelTable {
    pub fn new() -> Self {
        LabelTable {
            labels: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: &str, address: u32) {
        self.labels.insert(name.to_string(), address);
    }

    pub fn get(&self, name: &str) -> u32 {
        match self.labels.get(name) {
            Some(address) => *address,
            None => panic!("Undefined label: {}", name),
        }
    }
}