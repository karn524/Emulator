#![allow(dead_code)]

use std::collections::HashMap;
use crate::assembler::emit::{
    emit_loadi,
    emit_add,
    emit_sub,
    emit_store,
    emit_hlt,
};

use crate::emulator::memory::Memory;

// 変数宣言
// int a; のような変数をレジスタに割り当てる
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

// 変数名からレジスタ番号を取り出す
fn get_register(
    variables: &HashMap<String, u8>,
    name: &str,
) -> u8 {
    match variables.get(name) {
        Some(reg) => *reg,
        None => panic!("Undefined variable: {}", name),
    }
}

// a = 3; のような代入
fn assign_number(
    memory: &mut Memory,
    pos: &mut u32,
    variables: &HashMap<String, u8>,
    name: &str,
    value: u32,
) {
    let reg = get_register(variables, name);

    emit_loadi(memory, pos, reg, value);
}

// a = a + b; のような加算
fn add_variables(
    memory: &mut Memory,
    pos: &mut u32,
    variables: &HashMap<String, u8>,
    dst_name: &str,
    src_name: &str,
) {
    let dst = get_register(variables, dst_name);
    let src = get_register(variables, src_name);

    emit_add(memory, pos, dst, src);
}

// a = a - b; のような減算
fn sub_variables(
    memory: &mut Memory,
    pos: &mut u32,
    variables: &HashMap<String, u8>,
    dst_name: &str,
    src_name: &str,
) {
    let dst = get_register(variables, dst_name);
    let src = get_register(variables, src_name);

    emit_sub(memory, pos, dst, src);
}

// store a, 400; のような保存
fn store_variable(
    memory: &mut Memory,
    pos: &mut u32,
    variables: &HashMap<String, u8>,
    name: &str,
    address: u32,
) {
    let reg = get_register(variables, name);

    emit_store(memory, pos, reg, address);
}

// C風ミニ言語の入口
pub fn compile_simple_c_program(memory: &mut Memory, pos: &mut u32) {
    let source = "
int a;
int b;
a = 3;
b = 1;
a = a + b;
a = a - b;
store a, 400;
";

    compile_source(memory, pos, source);
}

// C風コードを1行ずつ読む関数
pub fn compile_source(
    memory: &mut Memory,
    pos: &mut u32,
    source: &str,
) {
    let mut variables: HashMap<String, u8> = HashMap::new();

    for line in source.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        // int a;
        if line.starts_with("int ") {
            let name = line
                .trim_start_matches("int ")
                .trim_end_matches(";")
                .trim();

            declare_variable(&mut variables, name);

            println!("declare variable: {}", name);
            continue;
        }

        // a = 3;
        // b = 1;
        if line.contains("=") && !line.contains("+") && !line.contains("-") {
            let line_without_semicolon = line.trim_end_matches(";");

            let parts: Vec<&str> = line_without_semicolon.split("=").collect();

            if parts.len() != 2 {
                panic!("Invalid assignment: {}", line);
            }

            let name = parts[0].trim();

            let value: u32 = parts[1]
                .trim()
                .parse()
                .expect("Invalid number");

            assign_number(memory, pos, &variables, name, value);

            println!("assign number: {} = {}", name, value);
            continue;
        }

        // a = a + b;
        if line.contains("=") && line.contains("+") {
            let line_without_semicolon = line.trim_end_matches(";");

            let parts: Vec<&str> = line_without_semicolon.split("=").collect();

            if parts.len() != 2 {
                panic!("Invalid add expression: {}", line);
            }

            let dst_name = parts[0].trim();

            let right_side = parts[1].trim();
            let add_parts: Vec<&str> = right_side.split("+").collect();

            if add_parts.len() != 2 {
                panic!("Invalid add expression: {}", line);
            }

            let left_name = add_parts[0].trim();
            let src_name = add_parts[1].trim();

            if dst_name != left_name {
                panic!("Only a = a + b style is supported: {}", line);
            }

            add_variables(memory, pos, &variables, dst_name, src_name);

            println!("add variables: {} = {} + {}", dst_name, left_name, src_name);
            continue;
        }

        // a = a - b;
        if line.contains("=") && line.contains("-") {
            let line_without_semicolon = line.trim_end_matches(";");

            let parts: Vec<&str> = line_without_semicolon.split("=").collect();

            if parts.len() != 2 {
                panic!("Invalid sub expression: {}", line);
            }

            let dst_name = parts[0].trim();

            let right_side = parts[1].trim();
            let sub_parts: Vec<&str> = right_side.split("-").collect();

            if sub_parts.len() != 2 {
                panic!("Invalid sub expression: {}", line);
            }

            let left_name = sub_parts[0].trim();
            let src_name = sub_parts[1].trim();

            if dst_name != left_name {
                panic!("Only a = a - b style is supported: {}", line);
            }

            sub_variables(memory, pos, &variables, dst_name, src_name);

            println!("sub variables: {} = {} - {}", dst_name, left_name, src_name);
            continue;
        }

        // store a, 400;
        if line.starts_with("store ") {
            let line_without_semicolon = line
                .trim_start_matches("store ")
                .trim_end_matches(";")
                .trim();

            let parts: Vec<&str> = line_without_semicolon.split(",").collect();

            if parts.len() != 2 {
                panic!("Invalid store statement: {}", line);
            }

            let name = parts[0].trim();

            let address: u32 = parts[1]
                .trim()
                .parse()
                .expect("Invalid address");

            store_variable(memory, pos, &variables, name, address);

            println!("store variable: {}, {}", name, address);
            continue;
        }
        
        println!("compile line: {}", line);
    }

    emit_hlt(memory, pos);
}

pub fn compile_to_assembly(source: &str) -> String {
    let mut assembly = String::new();

    // 変数名とレジスタ番号の対応表
    // 例：a -> 0, b -> 1
    let mut variables: HashMap<String, u8> = HashMap::new();

    for line in source.lines() {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        println!("compile to assembly line: {}", line);

        // int a;
        // int b;
        // のような変数宣言を処理する
        if line.starts_with("int ") {
            let name = line
                .trim_start_matches("int ")
                .trim_end_matches(";")
                .trim();

            declare_variable(&mut variables, name);

            let reg = get_register(&variables, name);

            println!("declare variable for assembly: {} -> R{}", name, reg);

            continue;
        }

        // a = 3;
        // b = 1;
        // のような数値代入を処理する
        if line.contains("=") && !line.contains("+") && !line.contains("-") {
            let line_without_semicolon = line.trim_end_matches(";");

            let parts: Vec<&str> = line_without_semicolon.split("=").collect();

            if parts.len() != 2 {
                panic!("Invalid assignment: {}", line);
            }

            let name = parts[0].trim();

            let value: u32 = parts[1]
                .trim()
                .parse()
                .expect("Invalid number");

            let reg = get_register(&variables, name);

            assembly.push_str(&format!("LOADI R{}, {}\n", reg, value));

            println!("generate assembly: LOADI R{}, {}", reg, value);

            continue;
        }

        // a = a + b;
        // のような加算を処理する
        if line.contains("=") && line.contains("+") {
            let line_without_semicolon = line.trim_end_matches(";");

            let parts: Vec<&str> = line_without_semicolon.split("=").collect();

            if parts.len() != 2 {
                panic!("Invalid add expression: {}", line);
            }

            let dst_name = parts[0].trim();

            let right_side = parts[1].trim();
            let add_parts: Vec<&str> = right_side.split("+").collect();

            if add_parts.len() != 2 {
                panic!("Invalid add expression: {}", line);
            }

            let left_name = add_parts[0].trim();
            let src_name = add_parts[1].trim();

            if dst_name != left_name {
                panic!("Only a = a + b style is supported: {}", line);
            }

            let dst = get_register(&variables, dst_name);
            let src = get_register(&variables, src_name);

            assembly.push_str(&format!("ADD R{}, R{}\n", dst, src));

            println!("generate assembly: ADD R{}, R{}", dst, src);

            continue;
        }

        // a = a - b;
        // のような減算を処理する
        if line.contains("=") && line.contains("-") {
            let line_without_semicolon = line.trim_end_matches(";");

            let parts: Vec<&str> = line_without_semicolon.split("=").collect();

            if parts.len() != 2 {
                panic!("Invalid sub expression: {}", line);
            }

            let dst_name = parts[0].trim();

            let right_side = parts[1].trim();
            let sub_parts: Vec<&str> = right_side.split("-").collect();

            if sub_parts.len() != 2 {
                panic!("Invalid sub expression: {}", line);
            }

            let left_name = sub_parts[0].trim();
            let src_name = sub_parts[1].trim();

            if dst_name != left_name {
                panic!("Only a = a - b style is supported: {}", line);
            }

            let dst = get_register(&variables, dst_name);
            let src = get_register(&variables, src_name);

            assembly.push_str(&format!("SUB R{}, R{}\n", dst, src));

            println!("generate assembly: SUB R{}, R{}", dst, src);

            continue;
        }

                // store a, 400;
        // のような保存処理をアセンブリに変換する
        if line.starts_with("store ") {
            let line_without_semicolon = line
                .trim_start_matches("store ")
                .trim_end_matches(";")
                .trim();

            let parts: Vec<&str> = line_without_semicolon.split(",").collect();

            if parts.len() != 2 {
                panic!("Invalid store statement: {}", line);
            }

            let name = parts[0].trim();

            let address: u32 = parts[1]
                .trim()
                .parse()
                .expect("Invalid address");

            let reg = get_register(&variables, name);

            assembly.push_str(&format!("STORE R{}, {}\n", reg, address));

            println!("generate assembly: STORE R{}, {}", reg, address);

            continue;
        }
    }

    assembly.push_str("HLT\n");

    assembly
}