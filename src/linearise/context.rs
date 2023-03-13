use std::collections::HashSet;
use derive_new::new;
use crate::ertl::structure::Label;
use crate::linearise::x86_64::{Asm, AsmNode};

#[derive(new)]
pub struct Context<'a> {
    pub code: Asm<'a>,
    pub visited: HashSet<Label>,
    pub labels: HashSet<Label>,
}

impl<'a> Context<'a> {
    pub fn emit_at_label(&mut self, label: Label, node: AsmNode<'a>) {
        self.code.nodes.push(AsmNode::Label(label));
        self.code.nodes.push(node);
    }

    pub fn emit(&mut self, node: AsmNode<'a>) {
        self.code.nodes.push(node);
    }

    pub fn need_label(&mut self, label: Label) {
        self.labels.insert(label);
    }
}