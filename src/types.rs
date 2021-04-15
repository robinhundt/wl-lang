use crate::parser::Span;

pub enum Statement<'a> {
    Assignment(Ident<'a>, AExp<'a>, Span<'a>),
    Skip,
    If(BExp<'a>, Span<'a>, Box<Statement<'a>>, Box<Statement<'a>>),
    While(BExp<'a>, Span<'a>, Box<Statement<'a>>),
    Seq(Box<Statement<'a>>, Box<Statement<'a>>)
}

pub type Ident<'a> = Span<'a>;

pub enum AExp<'a> {
    Var(Ident<'a>),
    Num(i64),
    BinExp(Box<AExp<'a>>, AOp, Box<AExp<'a>>)
}

pub enum AOp {
    Plus,
    Minus,
    Times,
    Fraction
}

pub enum BExp<'a> {
    True,
    False,
    Not(Box<BExp<'a>>),
    BinAExp(Box<AExp<'a>>, ROp, Box<AExp<'a>>),
    BinBExp(Box<BExp<'a>>, BOp, Box<BExp<'a>>)
}

pub enum ROp {
    Lt,
    Gt,
    Leq,
    Geq,
    Eq
}

pub enum BOp {
    And,
    Or,
    Xor
}