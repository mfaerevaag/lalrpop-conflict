use std::fmt::{Debug, Formatter, Error};

pub type CProg = Vec<Box<CProgElem>>;

pub enum CProgElem {
    VarDecl(CType, CIdent),
    FuncProto(CType, CIdent, Vec<Box<CParam>>),
}

pub type CParam = (CType, CIdent);

pub enum CType {
    Int,
    Char,
    Void
}

pub type CIdent = String;
