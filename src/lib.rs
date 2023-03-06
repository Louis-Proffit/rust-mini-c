#![feature(assert_matches)]
extern crate nom;
extern crate logos;
extern crate logos_nom_bridge;

use std::rc::Rc;
use crate::common::Stdout;
use crate::ertl::{ertl_file, ErtlResult};
use crate::parser::{parse_file, ParserResult};
use crate::rtl::{rtl_file, RtlResult};
use crate::rtl::interpreter::{interp_rtl_file, RtlInterpreterResult};
use crate::typer::{typ_file, TypResult};
use crate::typer::context::FileContext;
use crate::typer::interpreter::{interp_typed_file, TyperInterpreterResult};

pub mod common;
pub mod parser;
pub mod typer;
pub mod rtl;
pub mod ertl;
pub mod utils;

pub fn minic_parse(input: &str) -> ParserResult {
    parse_file(input)
}

impl parser::structure::File<'_> {
    pub fn minic_typ(&self) -> TypResult<typer::structure::File> {
        typ_file(Rc::new(FileContext::default()), self)
    }
}


impl<'a> typer::structure::File<'a> {
    pub fn minic_rtl(&self) -> RtlResult<rtl::structure::File> {
        rtl_file(self)
    }

    pub fn minic_interp(&'a self) -> TyperInterpreterResult<Stdout> {
        interp_typed_file(self)
    }
}

impl rtl::structure::File<'_> {
    pub fn minic_ertl(&self) -> ErtlResult<ertl::structure::File> {
        ertl_file(self)
    }

    pub fn minic_interp(&self) -> RtlInterpreterResult<Stdout> {
        interp_rtl_file(self)
    }
}