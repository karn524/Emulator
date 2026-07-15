#![allow(dead_code)]

use std::collections::HashMap;
use crate::emulator::cpu::{LOAD, LOADI, STORE, ADD, SUB, MOV, JMP, JZ, JNZ, JS, JNS, PUSH, POP, CALL, RET, INT, IRET, HLT};
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

// INT
// interrupt_register に設定された割り込み処理へジャンプする
pub fn emit_int(memory: &mut Memory, pos: &mut u32) {
    memory.write_u8(*pos, INT);
    *pos += 1;
}

// IRET
// 割り込み処理から戻る
pub fn emit_iret(memory: &mut Memory, pos: &mut u32) {
    memory.write_u8(*pos, IRET);
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

fn parse_register(text: &str) -> u8 {
    let text = text.trim();

    if !text.starts_with("R") {
        panic!("Invalid register format: {}", text);
    }

    let number_text = &text[1..];

    let reg: u8 = number_text
        .parse()
        .expect("Invalid register number");

    if reg >= 8 {
        panic!("Invalid register: R{}", reg);
    }

    reg
}

pub fn assemble_source(
    memory: &mut Memory,
    pos: &mut u32,
    source: &str,
    labels: &mut LabelTable,
    patches: &mut Vec<Patch>,
) {
    //let _ = patches;

    for line in source.lines() {
        // // 以降はコメントとして消す
        let line = line.split("//").next().unwrap().trim();

        // 空行は無視する
        if line.is_empty() {
            continue;
        }

        // ラベル行を判定する
        if line.ends_with(":") {
            let label_name = line.trim_end_matches(":");

            println!("label found: {} at address {}", label_name, *pos);

            labels.define(label_name, *pos);

            continue;
        }

        println!("assemble line: {}", line);

        // 最初の空白で命令名と引数部分に分ける
        let parts: Vec<&str> = line.splitn(2, ' ').collect();

        let instruction = parts[0];

        if instruction == "HLT" {
            emit_hlt(memory, pos);
            continue;
        }

        if instruction == "RET" {
            emit_ret(memory, pos);
            continue;
        }

        if instruction == "INT" {
            emit_int(memory, pos);
            continue;
        }

        if instruction == "IRET" {
            emit_iret(memory, pos);
            continue;
        }

        if parts.len() < 2 {
            panic!("Missing operands: {}", line);
        }

        let operands_text = parts[1];

        let operands: Vec<&str> = operands_text
            .split(',')
            .map(|x| x.trim())
            .collect();

        match instruction {
            "LOADI" => {
                if operands.len() != 2 {
                    panic!("Invalid LOADI format: {}", line);
                }

                let reg = parse_register(operands[0]);
                let value: u32 = operands[1]
                    .parse()
                    .expect("Invalid LOADI value");

                emit_loadi(memory, pos, reg, value);
            }

            "ADD" => {
                if operands.len() != 2 {
                    panic!("Invalid ADD format: {}", line);
                }

                let dst = parse_register(operands[0]);
                let src = parse_register(operands[1]);

                emit_add(memory, pos, dst, src);
            }

            "SUB" => {
                if operands.len() != 2 {
                    panic!("Invalid SUB format: {}", line);
                }

                let dst = parse_register(operands[0]);
                let src = parse_register(operands[1]);

                emit_sub(memory, pos, dst, src);
            }

            "MOV" => {
                if operands.len() != 2 {
                    panic!("Invalid MOV format: {}", line);
                }

                let dst = parse_register(operands[0]);
                let src = parse_register(operands[1]);

                emit_mov(memory, pos, dst, src);
            }

            "STORE" => {
                if operands.len() != 2 {
                    panic!("Invalid STORE format: {}", line);
                }

                let reg = parse_register(operands[0]);

                let address: u32 = operands[1]
                    .parse()
                    .expect("Invalid STORE address");

                emit_store(memory, pos, reg, address);
            }

            "LOAD" => {
                if operands.len() != 2 {
                    panic!("Invalid LOAD format: {}", line);
                }

                let reg = parse_register(operands[0]);

                let address: u32 = operands[1]
                    .parse()
                    .expect("Invalid LOAD address");

                emit_load(memory, pos, reg, address);
            }

            "PUSH" => {
                if operands.len() != 1 {
                    panic!("Invalid PUSH format: {}", line);
                }

                let reg = parse_register(operands[0]);

                emit_push(memory, pos, reg);
            }

            "POP" => {
                if operands.len() != 1 {
                    panic!("Invalid POP format: {}", line);
                }

                let reg = parse_register(operands[0]);

                emit_pop(memory, pos, reg);
            }

            "RET" => {
                if operands.len() != 0 {
                    panic!("Invalid RET format: {}", line);
                }

                emit_ret(memory, pos);
            }

            "INT" => {
                if operands.len() != 0 {
                    panic!("Invalid INT format: {}", line);
                }

                emit_int(memory, pos);
            }

            "IRET" => {
                if operands.len() != 0 {
                    panic!("Invalid IRET format: {}", line);
                }

                emit_iret(memory, pos);
            }

            "CALL" => {
                if operands.len() != 1 {
                    panic!("Invalid CALL format: {}", line);
                }

                let target = operands[0];

                // CALL 12 のように数値なら、そのままemitする
                if let Ok(address) = target.parse::<u32>() {
                    emit_call(memory, pos, address);
                } else {
                    // CALL func のようにラベルなら、あとで解決する

                    // CALL命令を書き込む
                    memory.write_u8(*pos, CALL);
                    *pos += 1;

                    // アドレスを書き込む場所を保存する
                    let address_pos = *pos;

                    // いったん0を書いておく
                    memory.write_u32(*pos, 0);
                    *pos += 4;

                    // あとで label の実アドレスで書き換える
                    patches.push(Patch {
                        label: target.to_string(),
                        address_pos,
                    });

                    println!("CALL label patch: {} at address_pos {}", target, address_pos);
                }
            }

            "JMP" => {
                if operands.len() != 1 {
                    panic!("Invalid JMP format: {}", line);
                }

                let target = operands[0];

                if let Ok(address) = target.parse::<u32>() {
                    emit_jmp(memory, pos, address);
                } else {
                    memory.write_u8(*pos, JMP);
                    *pos += 1;

                    let address_pos = *pos;

                    memory.write_u32(*pos, 0);
                    *pos += 4;

                    patches.push(Patch {
                        label: target.to_string(),
                        address_pos,
                    });

                    println!("JMP label patch: {} at address_pos {}", target, address_pos);
                }
            }

            "JZ" => {
                if operands.len() != 1 {
                    panic!("Invalid JZ format: {}", line);
                }

                let target = operands[0];

                if let Ok(address) = target.parse::<u32>() {
                    emit_jz(memory, pos, address);
                } else {
                    memory.write_u8(*pos, JZ);
                    *pos += 1;

                    let address_pos = *pos;

                    memory.write_u32(*pos, 0);
                    *pos += 4;

                    patches.push(Patch {
                        label: target.to_string(),
                        address_pos,
                    });

                    println!("JZ label patch: {} at address_pos {}", target, address_pos);
                }
            }

            "JNZ" => {
                if operands.len() != 1 {
                    panic!("Invalid JNZ format: {}", line);
                }

                let target = operands[0];

                if let Ok(address) = target.parse::<u32>() {
                    emit_jnz(memory, pos, address);
                } else {
                    memory.write_u8(*pos, JNZ);
                    *pos += 1;

                    let address_pos = *pos;

                    memory.write_u32(*pos, 0);
                    *pos += 4;

                    patches.push(Patch {
                        label: target.to_string(),
                        address_pos,
                });

                    println!("JNZ label patch: {} at address_pos {}", target, address_pos);
                }
            }

            "JS" => {
                if operands.len() != 1 {
                    panic!("Invalid JS format: {}", line);
                }

                let target = operands[0];

                if let Ok(address) = target.parse::<u32>() {
                    emit_js(memory, pos, address);
                } else {
                    memory.write_u8(*pos, JS);
                    *pos += 1;

                    let address_pos = *pos;

                    memory.write_u32(*pos, 0);
                    *pos += 4;

                    patches.push(Patch {
                        label: target.to_string(),
                        address_pos,
                    });

                    println!("JS label patch: {} at address_pos {}", target, address_pos);
                }
            }   

            "JNS" => {
                if operands.len() != 1 {
                    panic!("Invalid JNS format: {}", line);
                }

                let target = operands[0];

                if let Ok(address) = target.parse::<u32>() {
                    emit_jns(memory, pos, address);
                } else {
                    memory.write_u8(*pos, JNS);
                    *pos += 1;

                    let address_pos = *pos;

                    memory.write_u32(*pos, 0);
                    *pos += 4;

                    patches.push(Patch {
                         label: target.to_string(),
                         address_pos,
                    });

                    println!("JNS label patch: {} at address_pos {}", target, address_pos);
                }
            }

            _ => {
                panic!("Unknown assembly instruction: {}", instruction);
            }
        }
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