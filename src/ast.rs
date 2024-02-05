use std::fmt;

/// block -> {stat} [laststat]
#[derive(Debug, PartialEq)]
pub struct Block {
	pub stats: Vec<Stat>,
}

/// stat ->  vars `=` explist |
///       functioncall |
///       do block end |
///       while exp do block end |
///       repeat block until exp |
///       if exp then block {elseif exp then block} [else block] end |
///       for Name `=` exp `,` exp [`,` exp] do block end |
///       for names in explist do block end |
///       function funcname funcbody |
///       local function Name funcbody |
///       local names [`=` explist]
/// laststat -> return [explist] | break
#[derive(Debug, PartialEq)]
pub enum Stat {
	Assignment(Assignment),
	FunctionCall(FunctionCall),
	DoBlock(Block),
	WhileBlock(WhileBlock),
	IfBlock(IfBlock),
	ForRange(ForRange),
	ForIn(ForIn),
	FunctionDef(FunctionDef),
	LocalFunctionDef(LocalFunctionDef),
	LocalAssignment(LocalAssignment),
	Break,
	Return(Vec<Expr>),
}

/// if exp then block {elseif exp then block} [else block] end
#[derive(Debug, PartialEq)]
pub struct IfBlock {
	pub expr: Expr,
	pub block: Block,
	pub elseif: Vec<ElseIf>,
	pub else_block: Option<Block>,
}

/// elseif exp then block
#[derive(Debug, PartialEq)]
pub struct ElseIf {
	pub expr: Expr,
	pub block: Block,
}

/// while exp do block end
#[derive(Debug, PartialEq)]
pub struct WhileBlock {
	pub expr: Expr,
	pub block: Block,
}

/// for Name `=` exp `,` exp [`,` exp] do block end
#[derive(Debug, PartialEq)]
pub struct ForRange {
	pub name: Name,
	pub exprs: (Expr, Expr, Option<Expr>),
	pub block: Block,
}

/// for names in explist do block end
#[derive(Debug, PartialEq)]
pub struct ForIn {
	pub names: Vec<Name>,
	pub exprs: Vec<Expr>,
	pub block: Block,
}

/// vars '=' explist
#[derive(Debug, PartialEq)]
pub struct Assignment {
	pub vars: Vec<SuffixExpr>,
	pub exprs: Vec<Expr>,
}

/// local names [`=` explist]
#[derive(Debug, PartialEq)]
pub struct LocalAssignment {
	pub names: Vec<Name>,
	pub exprs: Vec<Expr>, // If vec is empty there is no `=`
}

/// funcname -> Name {`.` Name} [`:` Name]
#[derive(Debug, PartialEq)]
pub struct FuncName {
	pub path: Vec<Name>,
	pub method: Option<Name>,
}

/// primary_exp -> Name | '(' expr ')'
#[derive(Debug, PartialEq)]
pub enum PrimaryExpr {
	Name(Name),
	Expr(Box<Expr>),
}

/// expr -> nil | Bool | Numeral | String
///       |  tableconstructor | FUNCTION body | suffix_exp
///       |  exp binop exp | unop exp
/// tableconstructor -> `{` [fieldlist] `}`
#[derive(Debug, PartialEq)]
pub enum Expr {
	Nil,
	Bool(bool),
	Num(f64),
	Str(String),
	Lambda(FuncBody),
	Table(Vec<Field>),
	BinExp(BinExp),
	UnExp(UnExp),
	SuffixExpr(Box<SuffixExpr>),
}

/// suffix_exp -> primary_exp { suffix }
#[derive(Debug, PartialEq)]
pub struct SuffixExpr {
	pub exp: PrimaryExpr,
	pub suffix: Vec<Suffix>,
}

/// suffix -> `.` Name
///         | `[` exp `]`
///         | args
/// args ->  `(` [explist] `)`
#[derive(Debug, PartialEq)]
pub enum Suffix {
	Property(Name),
	Index(Expr),
	Call(Vec<Expr>),
}

#[derive(Debug, PartialEq)]
pub struct FunctionCall {
	pub expr: SuffixExpr,
	pub args: Vec<Expr>,
}

/// function funcname funcbody
#[derive(Debug, PartialEq)]
pub struct FunctionDef {
	pub name: FuncName,
	pub body: FuncBody,
}

/// local function Name funcbody
#[derive(Debug, PartialEq)]
pub struct LocalFunctionDef {
	pub name: Name,
	pub body: FuncBody,
}

/// funcbody -> `(` [parlist] `)` block end
/// parlist -> names [`,`]
#[derive(Debug, PartialEq)]
pub struct FuncBody {
	pub params: Vec<Name>,
	pub body: Block,
}

/// field -> `[` exp `]` | `=` exp | Name `=` exp | exp
#[derive(Debug, PartialEq)]
pub enum Field {
	Assign(Name, Expr),
	ExprAssign(Expr, Expr),
	Expr(Expr),
}

/// Name
#[derive(PartialEq, Debug)]
pub struct Name(pub String);

/// exp binop exp
#[derive(Debug, PartialEq)]
pub struct BinExp {
	pub op: BinOp,
	pub lhs: Box<Expr>,
	pub rhs: Box<Expr>,
}

/// unop exp
#[derive(Debug, PartialEq)]
pub struct UnExp {
	pub op: UnOp,
	pub exp: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub enum BinOp {
	Plus,
	Minus,
	Mul,
	Div,
	Pow,
	Mod,
	Concat,
	Lt,
	Lte,
	Gt,
	Gte,
	Eq,
	Neq,
	And,
	Or,
}

impl BinOp {
	pub fn priority(&self) -> i32 {
		match self {
			BinOp::Pow => 8,
			BinOp::Mul | BinOp::Div | BinOp::Mod => 6,
			BinOp::Plus | BinOp::Minus => 5,
			BinOp::Concat => 4,
			BinOp::Lt | BinOp::Gt | BinOp::Lte | BinOp::Gte | BinOp::Eq | BinOp::Neq => 3,
			BinOp::And => 2,
			BinOp::Or => 1,
		}
	}
}

/// unop -> `-` | not | `#` | `~`
#[derive(Debug, PartialEq)]
pub enum UnOp {
	Minus,
	Not,
	Len,
}

impl UnOp {
	pub fn priority(&self) -> i32 {
		7
	}
}

impl fmt::Display for BinOp {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				BinOp::Pow => "^",
				BinOp::Mul => "*",
				BinOp::Div => "*",
				BinOp::Mod => "%",
				BinOp::Plus => "+",
				BinOp::Minus => "-",
				BinOp::Concat => "..",
				BinOp::Lt => "<",
				BinOp::Gt => ">",
				BinOp::Lte => "<=",
				BinOp::Gte => "<=",
				BinOp::Eq => "==",
				BinOp::Neq => "~=",
				BinOp::And => "and",
				BinOp::Or => "or",
			}
		)
	}
}

impl fmt::Display for UnOp {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				UnOp::Minus => "-",
				UnOp::Not => "not",
				UnOp::Len => "#",
			}
		)
	}
}
