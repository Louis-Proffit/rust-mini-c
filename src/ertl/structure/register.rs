use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum Register {
    Virtual(crate::rtl::structure::register::Register),
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

pub const PARAMETERS: [Register; 6] = [
    Register::Rdi,
    Register::Rsi,
    Register::Rdx,
    Register::Rcx,
    Register::R8,
    Register::R9
];

pub const RESULT: Register = Register::Rax;


pub const CALLER_SAVED: [Register; 7] = [
    Register::Rax,
    Register::Rdi,
    Register::Rsi,
    Register::Rdx,
    Register::Rcx,
    Register::R8,
    Register::R9
];

pub const CALLEE_SAVED: [Register; 5] = [
    Register::Rbx,
    Register::R12,
    Register::R13,
    Register::R14,
    Register::R15
];

pub const ALLOCATABLE: [Register; 12] = [
    Register::Rax,
    Register::Rdi,
    Register::Rsi,
    Register::Rdx,
    Register::Rcx,
    Register::R8,
    Register::R9,
    Register::Rbx,
    Register::R12,
    Register::R13,
    Register::R14,
    Register::R15
];

pub const TMP_1: Register = Register::R10;
pub const TMP_2: Register = Register::R11;

impl Into<Register> for crate::rtl::structure::register::Register {
    fn into(self) -> Register {
        Register::Virtual(self)
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::Virtual(r) => write!(f, "{}", r),
            Register::Rax => write!(f, "rax"),
            Register::Rdi => write!(f, "rdi"),
            Register::Rdx => write!(f, "rdx"),
            Register::Rbp => write!(f, "rbp"),
            Register::Rsi => write!(f, "rsi"),
            Register::Rcx => write!(f, "rcx"),
            Register::Rsp => write!(f, "rsp"),
            Register::Rbx => write!(f, "rbx"),
            Register::R8 => write!(f, "r8"),
            Register::R9 => write!(f, "r9"),
            Register::R10 => write!(f, "r10"),
            Register::R11 => write!(f, "r11"),
            Register::R12 => write!(f, "r12"),
            Register::R13 => write!(f, "r13"),
            Register::R14 => write!(f, "r14"),
            Register::R15 => write!(f, "r15")
        }
    }
}