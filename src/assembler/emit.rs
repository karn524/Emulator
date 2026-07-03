#![allow(dead_code)]

use std::collections::HashMap;
use crate::emulator::cpu::{LOADI, STORE, ADD, SUB, MOV, JMP, JZ, JNZ, HLT};
use crate::emulator::memory::Memory;

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

// JMP label
// label の番地はあとで解決する
pub fn emit_jmp_label(
    memory: &mut Memory,
    pos: &mut u32,
    label: &str,
    patches: &mut Vec<Patch>,
) {
    memory.write_u8(*pos, JMP);
    *pos += 1;

    let address_pos = *pos;
    memory.write_u32(*pos, 0);
    *pos += 4;

    patches.push(Patch {
        label: label.to_string(),
        address_pos,
    });
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

pub struct Patch {
    pub label: String,
    pub address_pos: u32,
}

// JZ Rn, label
// label の番地はまだ分からないので、仮に0を書いて、あとで修正する
pub fn emit_jz_label(
    memory: &mut Memory,
    pos: &mut u32,
    reg: u8,
    label: &str,
    patches: &mut Vec<Patch>,
) {
    memory.write_u8(*pos, JZ);
    *pos += 1;

    memory.write_u8(*pos, reg);
    *pos += 1;

    let address_pos = *pos;
    memory.write_u32(*pos, 0);
    *pos += 4;

    patches.push(Patch {
        label: label.to_string(),
        address_pos,
    });
}

pub fn resolve_patches(
    memory: &mut Memory,
    labels: &LabelTable,
    patches: &[Patch],
) {
    for patch in patches {
        let address = labels.get(&patch.label);
        memory.write_u32(patch.address_pos, address);
    }
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