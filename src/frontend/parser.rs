use super::lexer::Token;
/*
 * Program = Statements*
 *
 * Statements = Statement*
 *
 *
 * Statement = CoreStmt ";"
 *
 * CoreStmt =
 *      | ImportStatement
 *      | FuncDefStmt
 *      | ImplBlock
 *      | PrimaryExpr
 *      | ReturnStatement
 *
 * Block = "{" Statements "}"
 * Type =
 *      | int32_t
 *      | "str"
 *      | ()
 *
 * ImportStatement = "use" Identifier ("::" Identifer)*
 * FuncDefStmt = fn Identifier "(" (Identifier (: Type )?)* ")"
 * ImplBlock = impl Identifier Block
 *
 * ReturnStatement = "return" PrimaryExpr
 *
 * BinaryOps =
 *      | !
 *      | +
 *      | ||
 *      | -
 *      | *
 *      | **
 *      | /
 *
 *
 * Number : digit+ (dot digit+)?
 *
 * Boolean = True | False
 * PrimaryExpr =
 *      | Number
 *      | Boolean
 *      | String
 *      | BinaryExpr
 *
 *  BinaryExpr =  PrimaryExpr BinaryOps PrimaryExpr
 *
 */
// A few type definitions to be used by our parsers below

pub trait ProvidingLocation {
    fn get_location() -> logos::Span;
}

pub struct Program {
    location: logos::Span,
}

pub struct Cell {
    location: logos::Span,
}

pub enum ExprKind {
    Number(f64),
}
pub struct PrimitiveExpr {
    location: logos::Span,
}

pub struct IfElseExpr {
    location: logos::Span,
}

pub struct NumberExpr {
    location: logos::Span,
}

pub struct StringExpr {
    location: logos::Span,
}

pub enum BinaryOp {
    Add,
    Minus,
    EqualEqual,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}
pub struct BinaryExpr {
    location: logos::Span,
    pri1: PrimitiveExpr,
    op: BinaryOp,
    pri2: PrimitiveExpr,
}

pub fn parse(s: &String) {}
