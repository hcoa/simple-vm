use std::{collections::HashMap};
use std::fs::read_to_string;


struct Vm {
    registers: HashMap<String, i32>,
}

impl Vm {
    fn new() -> Self {
        Vm { registers: HashMap::new()}
    }

    fn mov_const(&mut self, x: &str, y: i32) {
        self.registers.insert(x.to_string(), y);
    }

    fn mov(&mut self, x: &str, y: &str) {
        match self.registers.get(y) {
            Some(val_y) =>{
                 self.registers.insert(x.to_string(), *val_y);
            }
            _ => panic!("Register {y} is not initialised")
        }
    }

    fn add(&mut self, x: &str, y: &str) {
        match (self.registers.get(x), self.registers.get(y)) {
            (Some(val_x), Some(val_y)) => {
                let res = val_x.wrapping_add(*val_y);
                self.registers.insert(x.to_string(), res);
            },
            _ => panic!("One of the parameter's registry is missing")
        }
    }

    fn print(&self, x: &str) {
        match self.registers.get(x) {
            Some(val_x) => {
                // todo check if val_x >= 0;
                let ch = char::from_u32(*val_x as u32).expect("Failed to convert value: {val_x} to u32");
            
                print!("{ch}");
            },
            None => ()
        }
    }
    
}
/*
struct Register(String);
struct Constant(i32);

enum Instruction {
    Mov(Register, Register),
    MovConst(Register, Constant),
    Add(Register, Register),
    Print(Register)
}
*/

fn parse_exec(commands: Vec<&str>) {
    let mut vm = Vm::new();

    for command in commands {
        let mut parts = command.split_ascii_whitespace();
        let inst = parts.next();

        let x = parts.next();
        let y = parts.next();
        match (inst, x, y) {
            (Some("mov"), Some(value_x), Some(value_y)) => {
                match value_y.parse::<i32>() {
                    Ok(value) => vm.mov_const(value_x, value),
                    Err(_) => vm.mov(value_x, value_y),
                }        
            }
            (Some("add"), Some(val_x), Some(val_y)) => vm.add(val_x, val_y),
            (Some("print"), Some(x), None) => vm.print(x),
            (Some(other_inst), _, _) => panic!("{other_inst} is not implemented yet"),
            _ => panic!("Failed to parse a line"),
        }
    }
}

// fn validate(inp: String) -> Result<Vec<Errs>, Vec<Instruction>> {
//     unimplemented!();
// }

fn main() {
    // provide file name by stdin
    // open a file
    // parse lines
    // exec lines

    let args = std::env::args();
    let input = args.collect::<Vec<String>>();
    let file_name = match &input[..] {
        [_, file_name, ..] => file_name,
        _ => panic!("Usage: call it with file name")
    };
    
    let content = read_to_string(file_name).expect("Failed to read a file");

    let parts = content.as_str().split('\n').map(|ch| ch.trim()).collect::<Vec<&str>>();
    parse_exec(parts)
}




#[test]
fn super_quick_parse_exec_test() {
    let instructions = vec!["mov a 9999", "mov b -10", "add a b", "print a", "mov a 10", "print a"];
    
    parse_exec(instructions);
}
