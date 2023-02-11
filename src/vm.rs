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
