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

// 1行の前処理をする
// 前後の空白を消す
// 空行は空文字として返す
fn preprocess_line(line: &str) -> String {
    line.trim().to_string()
}

// int a;
// int b;
// のような変数宣言を処理する
// 処理できた場合は true を返す
// 処理できない場合は false を返す
fn compile_variable_declaration(
    line: &str,
    variables: &mut HashMap<String, u8>,
) -> bool {
    if !line.starts_with("int ") {
        return false;
    }

    let name = line
        .trim_start_matches("int ")
        .trim_end_matches(";")
        .trim();

    declare_variable(variables, name);

    let reg = get_register(variables, name);

    println!("declare variable for assembly: {} -> R{}", name, reg);

    true
}

// label loop;
// label end;
// のようなラベル定義をアセンブリに変換する
// 処理できた場合は true を返す
// 処理できない場合は false を返す
fn compile_label_statement(
    line: &str,
    assembly: &mut String,
) -> bool {
    if !line.starts_with("label ") {
        return false;
    }

    let label_name = line
        .trim_start_matches("label ")
        .trim_end_matches(";")
        .trim();

    assembly.push_str(&format!("{}:\n", label_name));

    println!("generate assembly label: {}:", label_name);

    true
}

// call func;
// のような関数呼び出しをアセンブリに変換する
// 処理できた場合は true を返す
// 処理できない場合は false を返す
fn compile_call_statement(
    line: &str,
    assembly: &mut String,
) -> bool {
    if !line.starts_with("call ") {
        return false;
    }

    let label_name = line
        .trim_start_matches("call ")
        .trim_end_matches(";")
        .trim();

    assembly.push_str(&format!("CALL {}\n", label_name));

    println!("generate assembly: CALL {}", label_name);

    true
}

// ret;
// enter;
// leave;
// interrupt;
// iret;
// のような引数なし命令をアセンブリに変換するための共通関数
fn compile_no_argument_statement(
    line: &str,
    keyword: &str,
    instruction: &str,
    assembly: &mut String,
) -> bool {
    let expected = format!("{};", keyword);

    if line != expected {
        return false;
    }

    assembly.push_str(&format!("{}\n", instruction));

    println!("generate assembly: {}", instruction);

    true
}

// jump loop;
// のような無条件ジャンプをアセンブリに変換する
// 処理できた場合は true を返す
// 処理できない場合は false を返す
fn compile_jump_statement(
    line: &str,
    assembly: &mut String,
) -> bool {
    if !line.starts_with("jump ") {
        return false;
    }

    let label_name = line
        .trim_start_matches("jump ")
        .trim_end_matches(";")
        .trim();

    assembly.push_str(&format!("JMP {}\n", label_name));

    println!("generate assembly: JMP {}", label_name);

    true
}

// jz end;
// jnz loop;
// js minus;
// jns plus;
// のような条件ジャンプをアセンブリに変換するための共通関数
fn compile_conditional_jump_statement(
    line: &str,
    keyword: &str,
    instruction: &str,
    assembly: &mut String,
) -> bool {
    let prefix = format!("{} ", keyword);

    if !line.starts_with(&prefix) {
        return false;
    }

    let label_name = line
        .trim_start_matches(&prefix)
        .trim_end_matches(";")
        .trim();

    assembly.push_str(&format!("{} {}\n", instruction, label_name));

    println!("generate assembly: {} {}", instruction, label_name);

    true
}

// ifz end;
// のような簡易if文を JZ に変換する
// 処理できた場合は true を返す
// 処理できない場合は false を返す
fn compile_ifz_statement(
    line: &str,
    assembly: &mut String,
) -> bool {
    if !line.starts_with("ifz ") {
        return false;
    }

    let label_name = line
        .trim_start_matches("ifz ")
        .trim_end_matches(";")
        .trim();

    assembly.push_str(&format!("JZ {}\n", label_name));

    println!("generate assembly: JZ {}", label_name);

    true
}

// ifnz loop;
// のような簡易if文を JNZ に変換する
// 処理できた場合は true を返す
// 処理できない場合は false を返す
fn compile_ifnz_statement(
    line: &str,
    assembly: &mut String,
) -> bool {
    if !line.starts_with("ifnz ") {
        return false;
    }

    let label_name = line
        .trim_start_matches("ifnz ")
        .trim_end_matches(";")
        .trim();

    assembly.push_str(&format!("JNZ {}\n", label_name));

    println!("generate assembly: JNZ {}", label_name);

    true
}

// ifs minus;
// のような簡易if文を JS に変換する
// 処理できた場合は true を返す
// 処理できない場合は false を返す
fn compile_ifs_statement(
    line: &str,
    assembly: &mut String,
) -> bool {
    if !line.starts_with("ifs ") {
        return false;
    }

    let label_name = line
        .trim_start_matches("ifs ")
        .trim_end_matches(";")
        .trim();

    assembly.push_str(&format!("JS {}\n", label_name));

    println!("generate assembly: JS {}", label_name);

    true
}

// ifns plus;
// のような簡易if文を JNS に変換する
// 処理できた場合は true を返す
// 処理できない場合は false を返す
fn compile_ifns_statement(
    line: &str,
    assembly: &mut String,
) -> bool {
    if !line.starts_with("ifns ") {
        return false;
    }

    let label_name = line
        .trim_start_matches("ifns ")
        .trim_end_matches(";")
        .trim();

    assembly.push_str(&format!("JNS {}\n", label_name));

    println!("generate assembly: JNS {}", label_name);

    true
}

// a = 3;
// b = 1;
// のような数値代入をアセンブリに変換する
// 処理できた場合は true を返す
// 処理できない場合は false を返す
fn compile_number_assignment(
    line: &str,
    variables: &HashMap<String, u8>,
    assembly: &mut String,
) -> bool {
    if !(line.contains("=") && !line.contains("+") && !line.contains("-")) {
        return false;
    }

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

    let reg = get_register(variables, name);

    assembly.push_str(&format!("LOADI R{}, {}\n", reg, value));

    println!("generate assembly: LOADI R{}, {}", reg, value);

    true
}

// a = a + b;
// のような加算をアセンブリに変換する
// 処理できた場合は true を返す
// 処理できない場合は false を返す
fn compile_add_expression(
    line: &str,
    variables: &HashMap<String, u8>,
    assembly: &mut String,
) -> bool {
    if !(line.contains("=") && line.contains("+")) {
        return false;
    }

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

    let dst = get_register(variables, dst_name);
    let src = get_register(variables, src_name);

    assembly.push_str(&format!("ADD R{}, R{}\n", dst, src));

    println!("generate assembly: ADD R{}, R{}", dst, src);

    true
}

// a = a - b;
// のような減算をアセンブリに変換する
// 処理できた場合は true を返す
// 処理できない場合は false を返す
fn compile_sub_expression(
    line: &str,
    variables: &HashMap<String, u8>,
    assembly: &mut String,
) -> bool {
    if !(line.contains("=") && line.contains("-")) {
        return false;
    }

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

    let dst = get_register(variables, dst_name);
    let src = get_register(variables, src_name);

    assembly.push_str(&format!("SUB R{}, R{}\n", dst, src));

    println!("generate assembly: SUB R{}, R{}", dst, src);

    true
}

// move b, a;
// のようなコピー処理をアセンブリに変換する
// 処理できた場合は true を返す
// 処理できない場合は false を返す
fn compile_move_statement(
    line: &str,
    variables: &HashMap<String, u8>,
    assembly: &mut String,
) -> bool {
    if !line.starts_with("move ") {
        return false;
    }

    let line_without_semicolon = line
        .trim_start_matches("move ")
        .trim_end_matches(";")
        .trim();

    let parts: Vec<&str> = line_without_semicolon.split(",").collect();

    if parts.len() != 2 {
        panic!("Invalid move statement: {}", line);
    }

    let dst_name = parts[0].trim();
    let src_name = parts[1].trim();

    let dst = get_register(variables, dst_name);
    let src = get_register(variables, src_name);

    assembly.push_str(&format!("MOV R{}, R{}\n", dst, src));

    println!("generate assembly: MOV R{}, R{}", dst, src);

    true
}

// compare a, b;
// のような比較文を CMP に変換する
// 処理できた場合は true を返す
// 処理できない場合は false を返す
fn compile_compare_statement(
    line: &str,
    variables: &HashMap<String, u8>,
    assembly: &mut String,
) -> bool {
    if !line.starts_with("compare ") {
        return false;
    }

    let line_without_semicolon = line
        .trim_start_matches("compare ")
        .trim_end_matches(";")
        .trim();

    let parts: Vec<&str> = line_without_semicolon.split(",").collect();

    if parts.len() != 2 {
        panic!("Invalid compare statement: {}", line);
    }

    let left_name = parts[0].trim();
    let right_name = parts[1].trim();

    let left = get_register(variables, left_name);
    let right = get_register(variables, right_name);

    assembly.push_str(&format!("CMP R{}, R{}\n", left, right));

    println!("generate assembly: CMP R{}, R{}", left, right);

    true
}

// load a, 400;
// のような読み込み処理をアセンブリに変換する
// 処理できた場合は true を返す
// 処理できない場合は false を返す
fn compile_load_statement(
    line: &str,
    variables: &HashMap<String, u8>,
    assembly: &mut String,
) -> bool {
    if !line.starts_with("load ") {
        return false;
    }

    let line_without_semicolon = line
        .trim_start_matches("load ")
        .trim_end_matches(";")
        .trim();

    let parts: Vec<&str> = line_without_semicolon.split(",").collect();

    if parts.len() != 2 {
        panic!("Invalid load statement: {}", line);
    }

    let name = parts[0].trim();

    let address: u32 = parts[1]
        .trim()
        .parse()
        .expect("Invalid address");

    let reg = get_register(variables, name);

    assembly.push_str(&format!("LOAD R{}, {}\n", reg, address));

    println!("generate assembly: LOAD R{}, {}", reg, address);

    true
}

// push a;
// のようなスタック保存処理をアセンブリに変換する
// 処理できた場合は true を返す
// 処理できない場合は false を返す
fn compile_push_statement(
    line: &str,
    variables: &HashMap<String, u8>,
    assembly: &mut String,
) -> bool {
    if !line.starts_with("push ") {
        return false;
    }

    let name = line
        .trim_start_matches("push ")
        .trim_end_matches(";")
        .trim();

    let reg = get_register(variables, name);

    assembly.push_str(&format!("PUSH R{}\n", reg));

    println!("generate assembly: PUSH R{}", reg);

    true
}

// pop b;
// のようなスタック復元処理をアセンブリに変換する
// 処理できた場合は true を返す
// 処理できない場合は false を返す
fn compile_pop_statement(
    line: &str,
    variables: &HashMap<String, u8>,
    assembly: &mut String,
) -> bool {
    if !line.starts_with("pop ") {
        return false;
    }

    let name = line
        .trim_start_matches("pop ")
        .trim_end_matches(";")
        .trim();

    let reg = get_register(variables, name);

    assembly.push_str(&format!("POP R{}\n", reg));

    println!("generate assembly: POP R{}", reg);

    true
}

// store a, 400;
// のような保存処理をアセンブリに変換する
// 処理できた場合は true を返す
// 処理できない場合は false を返す
fn compile_store_statement(
    line: &str,
    variables: &HashMap<String, u8>,
    assembly: &mut String,
) -> bool {
    if !line.starts_with("store ") {
        return false;
    }

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

    let reg = get_register(variables, name);

    assembly.push_str(&format!("STORE R{}, {}\n", reg, address));

    println!("generate assembly: STORE R{}, {}", reg, address);

    true
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
        let line = preprocess_line(line);

        if line.is_empty() {
            continue;
        }

        println!("compile to assembly line: {}", line);

        // 変数宣言の変換を関数化
        if compile_variable_declaration(&line, &mut variables) {
            continue;
        }

        // LABEL処理の変換を関数化
        if compile_label_statement(&line, &mut assembly) {
            continue;
        }

        // JMP処理の変換を関数化
        if compile_jump_statement(&line, &mut assembly) {
            continue;
        }

        // JZ処理の変換を関数化
        if compile_conditional_jump_statement(&line, "jz", "JZ", &mut assembly) {
            continue;
        }

        // IFZ処理の変換
        if compile_ifz_statement(&line, &mut assembly) {
            continue;
        }

        // JNZ処理の変換を関数化
        if compile_conditional_jump_statement(&line, "jnz", "JNZ", &mut assembly) {
            continue;
        }

        // IFNZ処理の変換
        if compile_ifnz_statement(&line, &mut assembly) {
            continue;
        }

        // JS処理の変換を関数化
        if compile_conditional_jump_statement(&line, "js", "JS", &mut assembly) {
            continue;
        }

        // IFS処理の変換
        if compile_ifs_statement(&line, &mut assembly) {
            continue;
        }

        // JNS処理の変換を関数化
        if compile_conditional_jump_statement(&line, "jns", "JNS", &mut assembly) {
            continue;
        }

        // IFNS処理の変換
        if compile_ifns_statement(&line, &mut assembly) {
            continue;
        }

        // CALL処理の変換を関数化
        if compile_call_statement(&line, &mut assembly) {
            continue;
        }

        // RET処理の変換を関数化
        if compile_no_argument_statement(&line, "ret", "RET", &mut assembly) {
            continue;
        }

        // ENTER処理の変換を関数化
        if compile_no_argument_statement(&line, "enter", "ENTER", &mut assembly) {
            continue;
        }

        // LEAVE処理の変換を関数化
        if compile_no_argument_statement(&line, "leave", "LEAVE", &mut assembly) {
            continue;
        }

        // INT処理の変換を関数化
        if compile_no_argument_statement(&line, "interrupt", "INT", &mut assembly) {
            continue;
        }

        // IRET処理の変換を関数化
        if compile_no_argument_statement(&line, "iret", "IRET", &mut assembly) {
            continue;
        }

        // 代入処理の変換を関数化
        if compile_number_assignment(&line, &variables, &mut assembly) {
            continue;
        }

        // 加算処理の変換を関数化
        if compile_add_expression(&line, &variables, &mut assembly) {
            continue;
        }

        // 減算処理の変換を関数化
        if compile_sub_expression(&line, &variables, &mut assembly) {
            continue;
        }

        // MOV処理の変換を関数化
        if compile_move_statement(&line, &variables, &mut assembly) {
            continue;
        }

        // compare処理の変換
        if compile_compare_statement(&line, &variables, &mut assembly) {
            continue;
        }

        // LOAD処理の変換を関数化
        if compile_load_statement(&line, &variables, &mut assembly) {
            continue;
        }

        // PUSH処理の変換を関数化
        if compile_push_statement(&line, &variables, &mut assembly) {
            continue;
        }

        // POP処理の変換を関数化
        if compile_pop_statement(&line, &variables, &mut assembly) {
            continue;
        }
        
        // STORE処理の変換を関数化
        if compile_store_statement(&line, &variables, &mut assembly) {
            continue;
        }

        panic!("Unsupported statement: {}", line);
    }

    assembly.push_str("HLT\n");

    assembly
}