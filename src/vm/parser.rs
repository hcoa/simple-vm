use std::{fmt::Display, str::FromStr};

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Register(String);

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Register {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Register(s.to_string()))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Constant(i32);

impl Constant {
    pub const ZERO: Constant = Constant(0);
}

impl std::ops::Add for Constant {
    type Output = Constant;

    fn add(self, rhs: Self) -> Self::Output {
        Constant(self.0 + rhs.0)
    }
}

impl From<i32> for Constant {
    fn from(value: i32) -> Self {
        Constant(value)
    }
}

impl FromStr for Constant {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = s.parse::<i32>()?;
        Ok(num.into())
    }
}

impl std::ops::Deref for Constant {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// encode all versions, then match,
// have a wrapper for Register/Constant
// parse.orElse
#[derive(Clone)]
pub enum ConstOrReg {
    Const(Constant),
    Reg(Register),
}

impl FromStr for ConstOrReg {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<Constant>().map_or(
            s.parse::<Register>().map(|reg| ConstOrReg::Reg(reg)),
            |cn| Ok(ConstOrReg::Const(cn)),
        )
    }
}

#[derive(Clone)]
pub enum Instruction {
    Mov(Register, ConstOrReg),
    Add(Register, Register),
    Jnz(ConstOrReg, ConstOrReg),
    Print(Register),
}

#[derive(Debug)]
// use thiserror to annotate with custom text
pub enum ParseError {
    EmptyInput,
    IncorrectArgument(String),
    InstructionNotFoundOrWrongArgs(String),
}

fn parse_token<T>(s: &str) -> Result<T, ParseError>
where
    T: FromStr,
    T::Err: Display,
{
    s.parse::<T>().map_err(|err| {
        ParseError::IncorrectArgument(format!("Failed to parse {s}, with error: {err}"))
    })
}

pub fn parse_instructions(input: Vec<&str>) -> Result<Vec<Instruction>, ParseError> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for (i, line) in input.iter().enumerate() {
        let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
        match parts[..] {
            ["mov", x, y] => {
                let x_reg = parse_token(x)?;
                let y_reg = parse_token(y)?;
                instructions.push(Instruction::Mov(x_reg, y_reg))
            }
            ["add", x, y] => {
                let x_reg = parse_token(x)?;
                let y_reg = parse_token(y)?;
                instructions.push(Instruction::Add(x_reg, y_reg))
            }
            ["print", x] => {
                let x_reg = parse_token(x)?;
                instructions.push(Instruction::Print(x_reg))
            }
            ["jnz", x, y] => {
                let x_reg = parse_token(x)?;
                let y_reg = parse_token(y)?;
                instructions.push(Instruction::Jnz(x_reg, y_reg))
            }
            [_, ..] => {
                return Result::Err(ParseError::InstructionNotFoundOrWrongArgs(format!(
                    "Not found instruction or wrong args on line {i}, error: {line}"
                )))
            }
            [] => return Result::Err(ParseError::EmptyInput),
        };
    }
    Ok(instructions)
}
