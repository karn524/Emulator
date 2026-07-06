use std::collections::HashMap;
use crate::assembler::emit::{
    emit_loadi,
    emit_add,
    emit_sub,
    emit_store,
    emit_hlt,
};

use crate::emulator::memory::Memory;

// C風ミニ言語の最初のコンパイル関数
//
// 想定するC風コード:
//
// int a;
// int b;
//
// a = 3;
// b = 1;
// a = a - b;
// store a, 400;
//
// これをCPU命令に変換する

fn declare_variable(
        variables: &mut HashMap<String, u8>,
        name: &str,
    ) {
        let reg = variables.len() as u8;

        if reg >= 8 {
            panic!("Too many variables");
        }

        variables.insert(name.to_string(), reg);
    }

fn assign_number(
    memory: &mut Memory,
    pos: &mut u32,
    variables: &HashMap<String, u8>,
    name: &str,
    value: u32,
) {
    let reg = variables[name];

    emit_loadi(memory, pos, reg, value);
}    

fn add_variables(
    memory: &mut Memory,
    pos: &mut u32,
    variables: &HashMap<String, u8>,
    dst_name: &str,
    src_name: &str,
) {
    let dst = variables[dst_name];
    let src = variables[src_name];

    emit_add(memory, pos, dst, src);
}

fn sub_variables(
    memory: &mut Memory,
    pos: &mut u32,
    variables: &HashMap<String, u8>,
    dst_name: &str,
    src_name: &str,
) {
    let dst = variables[dst_name];
    let src = variables[src_name];

    emit_sub(memory, pos, dst, src);
}

fn store_variable(
    memory: &mut Memory,
    pos: &mut u32,
    variables: &HashMap<String, u8>,
    name: &str,
    address: u32,
) {
    let reg = variables[name];

    emit_store(memory, pos, reg, address);
}

pub fn compile_simple_c_program(memory: &mut Memory, pos: &mut u32) {
    let mut variables: HashMap<String, u8> = HashMap::new();

    // int a;
    declare_variable(&mut variables, "a");

    // int b;
    declare_variable(&mut variables, "b");

    // a = 3;
    assign_number(memory, pos, &variables, "a", 3);

    // b = 1;
    assign_number(memory, pos, &variables, "b", 1);

    // a = a + b;
    add_variables(memory, pos, &variables, "a", "b");

    // a = a - b;
    sub_variables(memory, pos, &variables, "a", "b");

    // store a, 400;
    store_variable(memory, pos, &variables, "a", 400);

    emit_hlt(memory, pos);
}