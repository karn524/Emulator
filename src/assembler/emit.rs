#![allow(dead_code)]

use std::collections::HashMap;
use crate::emulator::cpu::{LOAD, LOADI, STORE, ADD, SUB, MOV, JMP, JZ, JNZ, JS, JNS, PUSH, POP, CALL, RET, HLT};
use crate::emulator::memory::Memory;

// LOAD Rn, address
pub fn emit_load(memory: &mut Memory, pos: &mut u32, reg: u8, address: u32) {
    memory.write_u8(*pos, LOAD);
    *pos += 1;

    memory.write_u8(*pos, reg);
    *pos += 1;

    memory.write_u32(*pos, address);
    *pos += 4;
}

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
// JZ address
pub fn emit_jz(memory: &mut Memory, pos: &mut u32, address: u32) {
    memory.write_u8(*pos, JZ);
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

// JNZ address
pub fn emit_jnz(memory: &mut Memory, pos: &mut u32, address: u32) {
    memory.write_u8(*pos, JNZ);
    *pos += 1;

    memory.write_u32(*pos, address);
    *pos += 4;
}

// JS address
// sign_flag が true なら address にジャンプする
pub fn emit_js(memory: &mut Memory, pos: &mut u32, address: u32) {
    memory.write_u8(*pos, JS);
    *pos += 1;

    memory.write_u32(*pos, address);
    *pos += 4;
}

// JNS address
// sign_flag が false なら address にジャンプする
pub fn emit_jns(memory: &mut Memory, pos: &mut u32, address: u32) {
    memory.write_u8(*pos, JNS);
    *pos += 1;

    memory.write_u32(*pos, address);
    *pos += 4;
}

// PUSH Rn
// レジスタの値をスタックに積む
pub fn emit_push(memory: &mut Memory, pos: &mut u32, reg: u8) {
    memory.write_u8(*pos, PUSH);
    *pos += 1;

    memory.write_u8(*pos, reg);
    *pos += 1;
}

// POP Rn
// スタックから値を取り出してレジスタに入れる
pub fn emit_pop(memory: &mut Memory, pos: &mut u32, reg: u8) {
    memory.write_u8(*pos, POP);
    *pos += 1;

    memory.write_u8(*pos, reg);
    *pos += 1;
}

// CALL address
// 戻り先アドレスをスタックに積んで、address にジャンプする
pub fn emit_call(memory: &mut Memory, pos: &mut u32, address: u32) {
    memory.write_u8(*pos, CALL);
    *pos += 1;

    memory.write_u32(*pos, address);
    *pos += 4;
}

// RET
// スタックから戻り先アドレスを取り出して戻る
pub fn emit_ret(memory: &mut Memory, pos: &mut u32) {
    memory.write_u8(*pos, RET);
    *pos += 1;
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

// JZ label
// zero_flag が true なら label にジャンプする
pub fn emit_jz_label(
    memory: &mut Memory,
    pos: &mut u32,
    label: &str,
    patches: &mut Vec<Patch>,
) {
    memory.write_u8(*pos, JZ);
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