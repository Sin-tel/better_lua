use crate::ast;
use crate::span::FileId;
use crate::symbol::SymbolId;

pub type TyId = usize;

#[derive(Debug)]
pub enum TyNode {
	Node(TyId),
	Ty(Ty),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
	Any,
	Err,
	Unit,
	TyVar,                      // type variable
	Free,                       // free type variable
	Module(FileId),             // imported module
	TyName(SymbolId),           // The type of the name of a type
	Named(SymbolId, Vec<TyId>), // A named type with it's associated types
	Fn(Vec<TyId>, TyId),        // args, ret
}

#[derive(Debug)]
pub enum TyAst {
	Any,
	Unit,
	Never,
	SelfTy,
	Named(ast::TyName, Vec<TyAst>),
	Array(Box<TyAst>),
	Fn(Vec<TyAst>, Box<TyAst>),
}
