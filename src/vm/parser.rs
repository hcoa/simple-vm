use std::{fmt::Display, str::FromStr};

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Register(String);

impl Register {
    pub fn of(r: String) -> Self {
        Register(r)
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct RegisterParseError;

impl Display for RegisterParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parsing failure, register value should be alphabetic")
    }
}

impl std::error::Error for RegisterParseError {}

impl FromStr for Register {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|c| c.is_alphabetic()) {
            Ok(Register::of(s.to_string()))
        } else {
            Result::Err(Box::new(RegisterParseError))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Constant(i32);

impl Constant {
    pub fn of(v: i32) -> Self {
        Constant(v)
    }
    pub const ZERO: Constant = Constant(0);
}

impl std::ops::Add for Constant {
    type Output = Constant;

    fn add(self, rhs: Self) -> Self::Output {
        Constant::of(self.0 + rhs.0)
    }
}

impl From<i32> for Constant {
    fn from(value: i32) -> Self {
        Constant::of(value)
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
#[derive(Clone, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
    Mov(Register, ConstOrReg),
    Add(Register, Register),
    Jnz(ConstOrReg, ConstOrReg),
    Print(Register),
}

#[derive(Debug, PartialEq)]
// use thiserror to annotate with custom text
pub enum ParseError {
    EmptyInput,
    EmptyLine,
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
    if input.is_empty() {
        return Result::Err(ParseError::EmptyInput);
    }
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
            [] => return Result::Err(ParseError::EmptyLine),
        };
    }
    Ok(instructions)
}

// ----- parser tests

#[cfg(test)]
mod tests {
    use super::*;
    use Instruction::*;

    #[test]
    fn test_parse_instructions() {
        let input = vec!["mov a 1", "mov b a", "jnz b 2", "add a b", "mov c 0"];
        let a = Register::of("a".to_string());
        let b = Register::of("b".to_string());
        let c = Register::of("c".to_string());

        let instructions = parse_instructions(input).unwrap();
        assert_eq!(
            instructions,
            vec![
                Mov(a.clone(), ConstOrReg::Const(Constant::of(1))),
                Mov(b.clone(), ConstOrReg::Reg(a.clone())),
                Jnz(
                    ConstOrReg::Reg(b.clone()),
                    ConstOrReg::Const(Constant::of(2))
                ),
                Add(a, b),
                Mov(c, ConstOrReg::Const(Constant::of(0))),
            ]
        );
    }

    #[test]
    fn test_unknown_instruction() {
        let instructions = parse_instructions(vec!["mov a 1", "mbx a 2"]);
        let actual_err = instructions.err().unwrap();

        // not the best way, remove checking of exact error message maybe
        assert_eq!(
            actual_err,
            ParseError::InstructionNotFoundOrWrongArgs(
                "Not found instruction or wrong args on line 1, error: mbx a 2".to_string()
            )
        )
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(
            parse_instructions(vec![]),
            Result::Err(ParseError::EmptyInput)
        )
    }

    #[test]
    fn test_empty_line_in_input() {
        assert_eq!(
            parse_instructions(vec!["mov a 1", ""]),
            Result::Err(ParseError::EmptyLine)
        )
    }

    #[test]
    fn test_incorrect_args() {
        assert_eq!(parse_instructions(vec!["mov 1 1"]), Result::Err(ParseError::IncorrectArgument("Failed to parse 1, with error: Parsing failure, register value should be alphabetic".to_string())))
    }
}
