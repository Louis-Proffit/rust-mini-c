use std::fmt::{Display, Formatter};
use derive_new::new;
use crate::common::{Ident, StackOffset, Value};
use crate::ertl::structure::Label;
use crate::ertl::structure::register::PhysicalRegister;
use crate::linearise::x86_64::mangle::mangle;
use crate::ltl::structure::Operand;

#[derive(new)]
pub struct Program<'a> {
    pub sections: Vec<Section<'a>>,
}

#[derive(new)]
pub enum Section<'a> {
    Text(Asm<'a>),
    Data(Asm<'a>),
}

#[derive(Debug)]
pub enum Size {
    B,
    W,
    L,
    Q,
}

/*
let reg r = fun fmt () -> fprintf fmt "%s" r
let ( ! % ) = reg
let imm i = fun fmt () -> fprintf fmt "$%i" i
let imm32 i = fun fmt () -> fprintf fmt "$%ld" i
let imm64 i = fun fmt () -> fprintf fmt "$%Ld" i
let ind ?(ofs=0) ? index ?(scale=1) r = fun fmt () -> match index with
| None -> fprintf fmt "%d(%s)" ofs r
| Some r1 -> fprintf fmt "%d(%s,%s,%d)" ofs r r1 scale
let abslab (l: label) = fun fmt () -> fprintf fmt "%a" mangle l
let rellab (l: label) = fun fmt () -> fprintf fmt "%a(%%rip)" mangle l
let lab = abslab
let ilab (l: label) = fun fmt () -> fprintf fmt "$%a" mangle l*/

#[derive(new)]
pub struct Asm<'a> {
    pub nodes: Vec<AsmNode<'a>>,
}

pub enum AsmNode<'a> {
    Mov(Size, X86Operand, X86Operand),
    Movz(Size, Size, X86Operand, X86Operand),
    Movs(Size, Size, X86Operand, X86Operand),
    Lea(Size, X86Operand, X86Operand),
    Inc(Size, X86Operand),
    Dec(Size, X86Operand),
    Neg(Size, X86Operand),
    Add(Size, X86Operand, X86Operand),
    Sub(Size, X86Operand, X86Operand),
    Imul(Size, X86Operand, X86Operand),
    IDivq(X86Operand),
    Cqto,
    Not(Size, X86Operand),
    And(Size, X86Operand, X86Operand),
    Or(Size, X86Operand, X86Operand),
    Xor(Size, X86Operand, X86Operand),
    Shr(Size, X86Operand, X86Operand),
    Shl(Size, X86Operand, X86Operand),
    Sar(Size, X86Operand, X86Operand),
    Jmp(Label),
    JmpStar(X86Operand),
    Call(Ident<'a>),
    CallStar(X86Operand),
    Leave,
    Ret,
    Je(Label),
    Jz(Label),
    Jne(Label),
    Jnz(Label),
    Js(Label),
    Jns(Label),
    Jg(Label),
    Jge(Label),
    Jl(Label),
    Jle(Label),
    Ja(Label),
    Jae(Label),
    Jb(Label),
    Jbe(Label),
    Cmp(Size, X86Operand, X86Operand),
    Test(Size, X86Operand, X86Operand),
    Sete(SizedPhysicalRegister),
    Setne(SizedPhysicalRegister),
    Sets(SizedPhysicalRegister),
    Setns(SizedPhysicalRegister),
    Setg(SizedPhysicalRegister),
    Setge(SizedPhysicalRegister),
    Setl(SizedPhysicalRegister),
    Setle(SizedPhysicalRegister),
    Seta(SizedPhysicalRegister),
    Setae(SizedPhysicalRegister),
    Setb(SizedPhysicalRegister),
    Setbe(SizedPhysicalRegister),
    Pushq(X86Operand),
    Popq(X86Operand),
    Align(u8),
    DByte(Vec<Value>),
    DInt(Vec<Value>),
    DWord(Vec<Value>),
    DQuad(Vec<Value>),
    String(Ident<'a>),
    Address(Vec<Label>),
    Space(u32),
    Label(Label),
    DeclFun(Ident<'a>),
    Globl(Ident<'a>),
    Comment(String),
}

pub struct SizedPhysicalRegister {
    pub register: PhysicalRegister,
    pub size: Size,
}

impl From<(PhysicalRegister, Size)> for SizedPhysicalRegister {
    fn from(value: (PhysicalRegister, Size)) -> Self {
        SizedPhysicalRegister { register: value.0, size: value.1 }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum X86Operand {
    Constant(Value),
    Register(PhysicalRegister),
    Offset(StackOffset, PhysicalRegister),
    OffsetScale(StackOffset, PhysicalRegister, PhysicalRegister, StackOffset),
}

impl Into<X86Operand> for Value {
    fn into(self) -> X86Operand {
        X86Operand::Constant(self)
    }
}

impl Into<X86Operand> for PhysicalRegister {
    fn into(self) -> X86Operand {
        X86Operand::Register(self)
    }
}

impl Into<X86Operand> for Operand {
    fn into(self) -> X86Operand {
        match self {
            Operand::Register(r) => X86Operand::Register(r),
            Operand::Spilled(o) => X86Operand::Offset(-8 * (o + 1), PhysicalRegister::Rbp)
        }
    }
}

pub trait X86: Display {}

impl Display for Program<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for section in &self.sections {
            write!(f, "{}", section)?;
        }
        Ok(())
    }
}

impl Display for Section<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Section::Text(asm) => {
                write!(f, "\t.text\n{}", asm)
            }
            Section::Data(asm) => {
                write!(f, "\t.data\n{}", asm)
            }
        }
    }
}

impl Display for Asm<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for node in &self.nodes {
            writeln!(f, "{}", node)?;
        }
        Ok(())
    }
}

impl Display for AsmNode<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AsmNode::Mov(s, f1, f2) => write!(f, "\tmov{} {}, {}", s, f1, f2),
            AsmNode::Movz(s1, s2, f1, f2) => write!(f, "\tmovz{}{} {}, {}", s1, s2, f1, f2),
            AsmNode::Movs(s1, s2, f1, f2) => write!(f, "\tmovs{}{} {}, {}", s1, s2, f1, f2),
            AsmNode::Lea(s, f1, f2) => write!(f, "\tlea{} {}, {}", s, f1, f2),
            AsmNode::And(s, f1, f2) => write!(f, "\tand{} {}, {}", s, f1, f2),
            AsmNode::Or(s, f1, f2) => write!(f, "\tor{} {}, {}", s, f1, f2),
            AsmNode::Xor(s, f1, f2) => write!(f, "\txor{} {}, {}", s, f1, f2),
            AsmNode::Shr(s, f1, f2) => write!(f, "\tshr{} {}, {}", s, f1, f2),
            AsmNode::Shl(s, f1, f2) => write!(f, "\tsjl{} {}, {}", s, f1, f2),
            AsmNode::Sar(s, f1, f2) => write!(f, "\tsar{} {}, {}", s, f1, f2),
            AsmNode::Add(s, f1, f2) => write!(f, "\tadd{} {}, {}", s, f1, f2),
            AsmNode::Sub(s, f1, f2) => write!(f, "\tsub{} {}, {}", s, f1, f2),
            AsmNode::Imul(s, f1, f2) => write!(f, "\timul{} {}, {}", s, f1, f2),
            AsmNode::Cmp(s, f1, f2) => write!(f, "\tcmp{} {}, {}", s, f1, f2),
            AsmNode::Test(s, f1, f2) => write!(f, "\ttest{} {}, {}", s, f1, f2),
            AsmNode::Inc(s, f1) => write!(f, "\tinc{} {}", s, f1),
            AsmNode::Dec(s, f1) => write!(f, "\tdec{} {}", s, f1),
            AsmNode::Neg(s, f1) => write!(f, "\tneg{} {}", s, f1),
            AsmNode::Not(s, f1) => write!(f, "\tnot{} {}", s, f1),
            AsmNode::IDivq(f1) => write!(f, "\tidivq {}", f1),
            AsmNode::Cqto => write!(f, "\tcqto"),
            AsmNode::Jmp(l) => write!(f, "\tjmp {}", l),
            AsmNode::JmpStar(l) => write!(f, "\tjmp *{}", l),
            AsmNode::Call(c) => write!(f, "\tcall {}", mangle(c)),
            AsmNode::CallStar(c) => write!(f, "\tcall *{}", c),
            AsmNode::Leave => write!(f, "\tleave"),
            AsmNode::Ret => write!(f, "\tret"),
            AsmNode::Je(f1) => write!(f, "\tje {}", f1),
            AsmNode::Jz(f1) => write!(f, "\tjz {}", f1),
            AsmNode::Jne(f1) => write!(f, "\tjne {}", f1),
            AsmNode::Jnz(f1) => write!(f, "\tjnz {}", f1),
            AsmNode::Js(f1) => write!(f, "\tjs {}", f1),
            AsmNode::Jns(f1) => write!(f, "\tjns {}", f1),
            AsmNode::Jg(f1) => write!(f, "\tjg {}", f1),
            AsmNode::Jge(f1) => write!(f, "\tjge {}", f1),
            AsmNode::Jl(f1) => write!(f, "\tjl {}", f1),
            AsmNode::Jle(f1) => write!(f, "\tjle {}", f1),
            AsmNode::Ja(f1) => write!(f, "\tja {}", f1),
            AsmNode::Jae(f1) => write!(f, "\tjae {}", f1),
            AsmNode::Jb(f1) => write!(f, "\tjb {}", f1),
            AsmNode::Jbe(f1) => write!(f, "\tjbe {}", f1),
            AsmNode::Sete(f1) => write!(f, "\tsete {}", f1),
            AsmNode::Setne(f1) => write!(f, "\tsetne {}", f1),
            AsmNode::Sets(f1) => write!(f, "\tsets {}", f1),
            AsmNode::Setns(f1) => write!(f, "\tsetns {}", f1),
            AsmNode::Setg(f1) => write!(f, "\tsetg {}", f1),
            AsmNode::Setge(f1) => write!(f, "\tsetge {}", f1),
            AsmNode::Setl(f1) => write!(f, "\tsetl {}", f1),
            AsmNode::Setle(f1) => write!(f, "\tsetle {}", f1),
            AsmNode::Seta(f1) => write!(f, "\tseta {}", f1),
            AsmNode::Setae(f1) => write!(f, "\tsetae {}", f1),
            AsmNode::Setb(f1) => write!(f, "\tsetb {}", f1),
            AsmNode::Setbe(f1) => write!(f, "\tsetbe {}", f1),
            AsmNode::Align(f1) => write!(f, "\t.align {}", f1),
            AsmNode::DByte(v) => write!(f, "\t.byte {}", CommaSeparatedVec(v)),
            AsmNode::DInt(v) => write!(f, "\t.int {}", CommaSeparatedVec(v)),
            AsmNode::DWord(v) => write!(f, "\t.word {}", CommaSeparatedVec(v)),
            AsmNode::DQuad(v) => write!(f, "\t.quad {}", CommaSeparatedVec(v)),
            AsmNode::String(f1) => write!(f, "\t.string {}", f1),
            AsmNode::Address(v) => write!(f, "\t.quad {}", CommaSeparatedVec(v)),
            AsmNode::Space(f1) => write!(f, "\t.space {}", f1),
            AsmNode::Pushq(f1) => write!(f, "\tpushq {}", f1),
            AsmNode::Popq(f1) => write!(f, "\tpopq {}", f1),
            AsmNode::Label(l) => write!(f, "{}:", mangle(l)),
            AsmNode::DeclFun(d) => write!(f, "{}:", d),
            AsmNode::Globl(d) => write!(f, "\t.globl {}", mangle(d)),
            AsmNode::Comment(c) => write!(f, "#{}", c),
        }
    }
}

impl Display for X86Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            X86Operand::Constant(v) => write!(f, "${}", v),
            X86Operand::Register(r) => write!(f, "{}", r),
            X86Operand::Offset(o, r) => write!(f, "{}({})", o, r),
            X86Operand::OffsetScale(o, r, s, n) => write!(f, "{}({},{},{})", o, r, s, n),
        }
    }
}

impl Display for SizedPhysicalRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "%")?;
        match &self.register {
            PhysicalRegister::Rax
            | PhysicalRegister::Rbx
            | PhysicalRegister::Rcx
            | PhysicalRegister::Rdx => {
                let core = match &self.register {
                    PhysicalRegister::Rax => "a",
                    PhysicalRegister::Rbx => "b",
                    PhysicalRegister::Rcx => "c",
                    PhysicalRegister::Rdx => "d",
                    _ => unreachable!()
                };
                match &self.size {
                    Size::B => write!(f, "{}l", core),
                    Size::W => write!(f, "{}x", core),
                    Size::L => write!(f, "e{}x", core),
                    Size::Q => write!(f, "r{}x", core),
                }
            }
            PhysicalRegister::Rdi
            | PhysicalRegister::Rbp
            | PhysicalRegister::Rsi
            | PhysicalRegister::Rsp => {
                let core = match &self.register {
                    PhysicalRegister::Rdi => "di",
                    PhysicalRegister::Rbp => "bp",
                    PhysicalRegister::Rsi => "si",
                    PhysicalRegister::Rsp => "sp",
                    _ => unreachable!()
                };
                match &self.size {
                    Size::B => write!(f, "{}l", core),
                    Size::W => write!(f, "{}", core),
                    Size::L => write!(f, "e{}", core),
                    Size::Q => write!(f, "r{}", core),
                }
            }
            PhysicalRegister::R8
            | PhysicalRegister::R9
            | PhysicalRegister::R10
            | PhysicalRegister::R11
            | PhysicalRegister::R12
            | PhysicalRegister::R13
            | PhysicalRegister::R14
            | PhysicalRegister::R15 => {
                let core = match &self.register {
                    PhysicalRegister::R8 => "r8",
                    PhysicalRegister::R9 => "r9",
                    PhysicalRegister::R10 => "r10",
                    PhysicalRegister::R11 => "r11",
                    PhysicalRegister::R12 => "r12",
                    PhysicalRegister::R13 => "r13",
                    PhysicalRegister::R14 => "r14",
                    PhysicalRegister::R15 => "r15",
                    _ => unreachable!()
                };
                match &self.size {
                    Size::B => write!(f, "{}b", core),
                    Size::W => write!(f, "{}w", core),
                    Size::L => write!(f, "{}d", core),
                    Size::Q => write!(f, "{}b", core),
                }
            }
        }
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::B => write!(f, "b"),
            Size::W => write!(f, "w"),
            Size::L => write!(f, "l"),
            Size::Q => write!(f, "q"),
        }
    }
}

struct CommaSeparatedVec<'a, T>(&'a Vec<T>);

impl<'a, T: Display> Display for CommaSeparatedVec<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.0.iter();
        match iter.next() {
            None => {}
            Some(x) => { write!(f, "{}", x)? }
        }

        for other in iter {
            write!(f, ", {}", other)?;
        }

        Ok(())
    }
}

mod mangle {
    use std::fmt::{Display, Formatter};
    use crate::linearise::x86_64::X86;

    pub fn mangle<T: Display>(item: T) -> impl X86 {
        MangleNone(item)
    }

    struct MangleNone<T: Display>(pub T);

    struct MangleLeadingUnderscore<T: Display>(pub T);

    impl<T: Display> Display for MangleNone<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl<T: Display> X86 for MangleNone<T> {}

    impl<T: Display> Display for MangleLeadingUnderscore<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "_{}", self.0)
        }
    }

    impl<T: Display> X86 for MangleLeadingUnderscore<T> {}
}
