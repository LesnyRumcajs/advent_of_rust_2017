use regex::Regex;
use std::collections::BTreeMap;

pub type RegisterName = String;

pub type Registers = BTreeMap<RegisterName, i32>;

#[derive(Debug, PartialEq, Eq)]
pub enum Operation {
    Inc,
    Dec,
}

impl Operation {
    fn from_str(s: &str) -> Option<Operation> {
        match s {
            "inc" => Some(Operation::Inc),
            "dec" => Some(Operation::Dec),
            _ => None,
        }
    }
}

pub struct Conditional {
    pub lhs: RegisterName,
    pub condition: Condition,
    pub rhs: i32,
}

impl Conditional {
    fn from_str(s: &str) -> Option<Conditional> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<reg>\w+) (?P<cond>[\\<\\>\\=\\!]+) (?P<val>[-\d]+$)").unwrap();
        }
        let captures = RE.captures(s)?;

        Some(Conditional {
            lhs: captures.name("reg")?.as_str().to_owned(),
            condition: Condition::from_str(&captures.name("cond")?.as_str().to_owned())?,
            rhs: captures.name("val")?.as_str().parse().unwrap(),
        })
    }

    pub fn check(&self, register: &Registers) -> bool {
        let lhs = *register.get(&self.lhs).unwrap();
        match self.condition {
            Condition::LT => lhs < self.rhs,
            Condition::LE => lhs <= self.rhs,
            Condition::GT => lhs > self.rhs,
            Condition::GE => lhs >= self.rhs,
            Condition::EQ => lhs == self.rhs,
            Condition::NE => lhs != self.rhs,
        }
    }
}

#[cfg(test)]
mod conditional_tests {
    use super::*;

    #[test]
    fn from_str() {
        assert!(Conditional::from_str("").is_none());
        assert!(Conditional::from_str("bxy <=> 489").is_none());

        let correct_conditional = Conditional::from_str("bxy <= -489").unwrap();
        assert_eq!(correct_conditional.lhs, "bxy");
        assert_eq!(correct_conditional.condition, Condition::LE);
        assert_eq!(correct_conditional.rhs, -489);
    }
}

#[derive(Eq, PartialEq, Debug)]
pub enum Condition {
    LT,
    LE,
    GT,
    GE,
    EQ,
    NE,
}

impl Condition {
    pub fn from_str(s: &str) -> Option<Condition> {
        match s {
            "<" => Some(Condition::LT),
            "<=" => Some(Condition::LE),
            ">" => Some(Condition::GT),
            ">=" => Some(Condition::GE),
            "==" => Some(Condition::EQ),
            "!=" => Some(Condition::NE),
            _ => None,
        }
    }
}

#[cfg(test)]
mod condition_tests {
    use crate::day8_helpers::model::Condition;

    #[test]
    fn from_str() {
        assert!(Condition::from_str("").is_none());
        assert!(Condition::from_str("<=>").is_none());
        assert_eq!(Condition::from_str("<"), Some(Condition::LT));
        assert_eq!(Condition::from_str("=="), Some(Condition::EQ));
        assert_eq!(Condition::from_str("!="), Some(Condition::NE));
    }
}

pub struct Instruction {
    pub register: RegisterName,
    pub op: Operation,
    pub op_val: i32,
    pub condition: Conditional,
}

impl Instruction {
    pub fn from_str(s: &str) -> Option<Instruction> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^(?P<reg>\w+) (?P<op>\w+) (?P<op_val>[-\d]+) if (?P<cond>.+)")
                    .unwrap();
        }
        let captures = RE.captures(s)?;
        Some(Instruction {
            register: captures.name("reg")?.as_str().to_owned(),
            op: Operation::from_str(captures.name("op")?.as_str())?,
            op_val: captures.name("op_val")?.as_str().parse().unwrap(),
            condition: Conditional::from_str(captures.name("cond")?.as_str())?,
        })
    }
}

#[cfg(test)]
mod instruction_tests {
    use super::*;

    #[test]
    fn from_str() {
        assert!(Instruction::from_str("").is_none());
        assert!(Instruction::from_str("m dec 430 if bxy <=> 489").is_none());

        let correct_instruction = Instruction::from_str("bxy inc 484 if e >= -709").unwrap();
        assert_eq!(correct_instruction.register, "bxy");
        assert_eq!(correct_instruction.op, Operation::Inc);
        assert_eq!(correct_instruction.op_val, 484);
        assert_eq!(correct_instruction.condition.lhs, "e");
        assert_eq!(correct_instruction.condition.condition, Condition::GE);
        assert_eq!(correct_instruction.condition.rhs, -709);
    }
}
