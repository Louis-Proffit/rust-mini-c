use std::fmt::{Display, Formatter};
use crate::rtl::structure::register::PseudoRegister;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub enum Register {
    Pseudo(PseudoRegister),
    Physical(PhysicalRegister),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub enum PhysicalRegister {
    Rax,
    Rdi,
    Rdx,
    Rbp,
    Rsi,
    Rcx,
    Rsp,
    Rbx,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}

pub const PARAMETERS: [PhysicalRegister; 6] = [
    PhysicalRegister::Rdi,
    PhysicalRegister::Rsi,
    PhysicalRegister::Rdx,
    PhysicalRegister::Rcx,
    PhysicalRegister::R8,
    PhysicalRegister::R9
];

pub const RESULT: PhysicalRegister = PhysicalRegister::Rax;


pub const CALLER_SAVED: [PhysicalRegister; 7] = [
    PhysicalRegister::Rax,
    PhysicalRegister::Rdi,
    PhysicalRegister::Rsi,
    PhysicalRegister::Rdx,
    PhysicalRegister::Rcx,
    PhysicalRegister::R8,
    PhysicalRegister::R9
];

pub const CALLEE_SAVED: [PhysicalRegister; 5] = [
    PhysicalRegister::Rbx,
    PhysicalRegister::R12,
    PhysicalRegister::R13,
    PhysicalRegister::R14,
    PhysicalRegister::R15
];

pub const ALLOCATABLE: [PhysicalRegister; 12] = [
    PhysicalRegister::Rax,
    PhysicalRegister::Rdi,
    PhysicalRegister::Rsi,
    PhysicalRegister::Rdx,
    PhysicalRegister::Rcx,
    PhysicalRegister::R8,
    PhysicalRegister::R9,
    PhysicalRegister::Rbx,
    PhysicalRegister::R12,
    PhysicalRegister::R13,
    PhysicalRegister::R14,
    PhysicalRegister::R15
];

pub const TMP_1: PhysicalRegister = PhysicalRegister::R10;
pub const TMP_2: PhysicalRegister = PhysicalRegister::R11;

impl Into<Register> for PseudoRegister {
    fn into(self) -> Register {
        Register::Pseudo(self)
    }
}

impl Display for PhysicalRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PhysicalRegister::Rax => write!(f, "rax"),
            PhysicalRegister::Rdi => write!(f, "rdi"),
            PhysicalRegister::Rdx => write!(f, "rdx"),
            PhysicalRegister::Rbp => write!(f, "rbp"),
            PhysicalRegister::Rsi => write!(f, "rsi"),
            PhysicalRegister::Rcx => write!(f, "rcx"),
            PhysicalRegister::Rsp => write!(f, "rsp"),
            PhysicalRegister::Rbx => write!(f, "rbx"),
            PhysicalRegister::R8 => write!(f, "r8"),
            PhysicalRegister::R9 => write!(f, "r9"),
            PhysicalRegister::R10 => write!(f, "r10"),
            PhysicalRegister::R11 => write!(f, "r11"),
            PhysicalRegister::R12 => write!(f, "r12"),
            PhysicalRegister::R13 => write!(f, "r13"),
            PhysicalRegister::R14 => write!(f, "r14"),
            PhysicalRegister::R15 => write!(f, "r15")
        }
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::Pseudo(r) => write!(f, "{}", r),
            Register::Physical(r) => write!(f, "{}", r),
        }
    }
}