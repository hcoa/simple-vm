use std::fs::read_to_string;

use vm::parser::parse_instructions;
mod vm;

fn main() {
    let args = std::env::args();
    let input = args.collect::<Vec<String>>();
    let file_name = match &input[..] {
        [_, file_name, ..] => file_name,
        _ => panic!("Usage: call it with file name"),
    };

    let content = read_to_string(file_name).expect("Failed to read a file");

    let parts = content
        .split('\n')
        .map(|ch| ch.trim())
        .collect::<Vec<&str>>();
    let instructions = parse_instructions(parts).unwrap();
    let mut vm = vm::Vm::new();
    vm.interpret(&instructions, 0);
}

#[test]
fn super_quick_parse_exec_test() {
    let instructions = vec![
        "mov a 9999",
        "mov b -10",
        "add a b",
        "print a",
        "mov a 10",
        "print a",
    ];

    let instructions = vm::parser::parse_instructions(instructions).unwrap();
    let mut vm = vm::Vm::new();
    vm.interpret(&instructions, 0);
}
