pub mod parser;

use std::collections::HashMap;

use self::parser::{ConstOrReg, Constant, Instruction, Register};

pub struct Vm<'a> {
    registers: HashMap<Register, Constant>,
    instructions: &'a [Instruction],
    pc: usize,
}

impl<'a> Vm<'a> {
    pub fn of(instructions: &'a [Instruction]) -> Self {
        Vm {
            registers: HashMap::new(),
            instructions,
            pc: 0,
        }
    }

    fn mov_const(&mut self, x: &Register, y: Constant) {
        self.registers.insert(x.clone(), y);
        self.pc += 1
    }

    fn mov(&mut self, x: &Register, y: &Register) {
        match self.registers.get(y) {
            Some(val_y) => {
                self.registers.insert(x.clone(), *val_y);
                self.pc += 1;
            }
            _ => panic!("Register {y} is not initialised"),
        }
    }

    fn add(&mut self, x: &Register, y: &Register) {
        match (self.registers.get(x), self.registers.get(y)) {
            (Some(val_x), Some(val_y)) => {
                let res: Constant = val_x.wrapping_add(**val_y).into();
                self.registers.insert(x.clone(), res);
                self.pc += 1;
            }
            _ => panic!("One of the register's parameters is missing"),
        }
    }

    fn print(&mut self, x: &Register) {
        match self.registers.get(x) {
            Some(val_x) => {
                if **val_x < 0 {
                    panic!("Value in register {x} is negative, failed to print it")
                }
                let ch = char::from_u32(**val_x as u32)
                    .expect(format!("Failed to convert value: {val_x} to u32").as_str());
                print!("{ch}");
                self.pc += 1;
            }
            None => (),
        }
    }

    fn jumpz(&mut self, x: &ConstOrReg, y: &ConstOrReg) {
        let value = match x {
            ConstOrReg::Const(constant) => constant,
            ConstOrReg::Reg(register) => &*self
                .registers
                .get(&register)
                .expect(format!("Rregister {register} must be initialized").as_str()),
        };
        if *value == Constant::ZERO {
            self.pc += 1;
            return;
        }
        let jump = match y {
            ConstOrReg::Const(constant) => constant,
            ConstOrReg::Reg(reg) => &*self
                .registers
                .get(&reg)
                .expect(format!("Register {reg} must be initialized").as_str()),
        };

        let new_pc = if *jump < Constant::ZERO {
            self.pc.checked_sub(jump.abs() as usize)
        } else {
            self.pc.checked_add(jump.abs() as usize)
        }
        .expect(format!("Could not jump {}", jump).as_str());
        if new_pc > self.instructions.len() {
            panic!("Trying to jump too far");
        }
        self.pc = new_pc;
    }

    pub fn interpret(&mut self, start_pc: usize) {
        self.pc = start_pc;
        loop {
            if let Some(instruction) = self.instructions.get(self.pc) {
                match instruction {
                    Instruction::Add(x, y) => self.add(&x, &y),
                    Instruction::Mov(x, y) => match y {
                        ConstOrReg::Const(constant) => self.mov_const(x, *constant),
                        ConstOrReg::Reg(reg) => self.mov(x, reg),
                    },
                    Instruction::Print(x) => self.print(&x),
                    Instruction::Jnz(x, y) => self.jumpz(&x, &y),
                }
            } else {
                return;
            }
        }
    }
}

// ------ vm tests

#[cfg(test)]
mod tests {
    use crate::vm::parser::{parse_instructions, Constant, Register};

    use super::Vm;

    #[test]
    fn test_mov() {
        let instructions = parse_instructions(vec!["mov a 1", "mov b a"]).unwrap();
        let a = Register::of("a".to_string());
        let b = Register::of("b".to_string());

        let mut vm = Vm::of(&instructions);
        vm.interpret(0);
        assert_eq!(vm.pc, 2);
        assert_eq!(*vm.registers.get(&a).unwrap(), Constant::of(1));
        assert_eq!(*vm.registers.get(&b).unwrap(), Constant::of(1));
    }

    #[test]
    fn test_add() {
        let instructions = parse_instructions(vec!["mov a 1", "mov b a", "add a b"]).unwrap();
        let a = Register::of("a".to_string());
        let b = Register::of("b".to_string());

        let mut vm = Vm::of(&instructions);
        vm.interpret(0);
        assert_eq!(vm.pc, 3);
        assert_eq!(*vm.registers.get(&a).unwrap(), Constant::of(2));
        assert_eq!(*vm.registers.get(&b).unwrap(), Constant::of(1));
    }

    // TODO add buffer for printing in vm
    // #[test]
    // fn check_print() {
    // }

    #[test]
    fn test_jump() {
        let instructions =
            parse_instructions(vec!["mov a 1", "mov b a", "jnz b 2", "add a b", "mov c 0"])
                .unwrap();
        let a = Register::of("a".to_string());
        let b = Register::of("b".to_string());
        let c = Register::of("c".to_string());

        let mut vm = Vm::of(&instructions);
        vm.interpret(0);
        assert_eq!(vm.pc, 5);
        assert_eq!(*vm.registers.get(&a).unwrap(), Constant::of(1));
        assert_eq!(*vm.registers.get(&b).unwrap(), Constant::of(1));
        assert_eq!(*vm.registers.get(&c).unwrap(), Constant::of(0));
    }

    #[test]
    fn test_backward_jump() {
        let instructions =
            parse_instructions(vec!["mov a 2", "mov b -1", "add a b", "jnz a -1"]).unwrap();

        let a = Register::of("a".to_string());
        let b = Register::of("b".to_string());
        let mut vm = Vm::of(&instructions);
        vm.interpret(0);
        assert_eq!(vm.pc, 4);
        assert_eq!(*vm.registers.get(&a).unwrap(), Constant::of(0));
        assert_eq!(*vm.registers.get(&b).unwrap(), Constant::of(-1));
    }
}
