use nom::{IResult, InputTakeAtPosition, AsChar};
use crate::types::{Statement, Ident, AExp, AOp, BExp};
use nom::do_parse;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::Parser;
use nom::multi::many1;
use nom::character::complete::{one_of, alpha1, space0, multispace0, char};
use nom::combinator::{recognize, map};
use nom::sequence::{separated_pair, delimited, terminated};
use nom::error::ParseError;
use nom_locate::{LocatedSpan, position};

pub type Span<'a> = LocatedSpan<&'a str>;

pub fn parse(input: &str) -> IResult<&str, Statement> {
    todo!()
}

fn parse_statement(input: Span) -> IResult<Span, Statement> {
    alt((
        parse_assignment,
        parse_skip,
        parse_if,
        parse_while,
        parse_seq
    ));
    todo!()
}


fn parse_assignment(i: Span) -> IResult<Span, Statement> {
    let (i, pos) = position(i)?;
    let (i, (ident, exp)) = separated_pair(
        parse_ident,
        ws(tag(":=")),
        parse_arithmetic_exp
    )(i)?;
    Ok((i, Statement::Assignment(ident, exp, pos)))
}

fn parse_ident(i: Span) -> IResult<Span, Ident> {
    recognize(alpha1)(i)
}

fn parse_skip(i: Span) -> IResult<Span, Statement> {
    tag("skip").map(|_| Statement::Skip).parse(i)
}

fn parse_if(i: Span) -> IResult<Span, Statement> {
    let (i, _) = tag("if")(i)?;
    let (i, pos) = position(i)?;
    let (i, exp) = ws(parse_bool_exp)(i)?;
    let (i, _) = tag("then").and_then(multispace0).parse(i)?;
    let (i, s1) = ws(parse_statement)(i)?;
    let (i, _) = tag("else").and_then(multispace0).parse(i)?;
    let (i, s2) = ws(parse_statement)(i)?;
    tag("fi")(i)?;
    Ok((i, Statement::If(exp, pos, Box::new(s1), Box::new(s2))))
}

fn parse_while(i: Span) -> IResult<Span, Statement> {
    let (i, _) = tag("while")(i)?;
    let (i, pos) = position(i)?;
    let (i, exp) = ws(parse_bool_exp)(i)?;
    let (i, _) = tag("do").and_then(multispace0).parse(i)?;
    let (i, s1) = ws(parse_statement)(i)?;
    tag("od")(i)?;
    Ok((i, Statement::While(exp, pos, Box::new(s1))))
}

fn parse_seq(i: Span) -> IResult<Span, Statement> {
    let (i, s1) = mws(terminated(parse_statement, char(';')))(i)?;
    let (i, s2) = parse_statement(i)?;
    Ok((i, Statement::Seq(Box::new(s1), Box::new(s2))))
}

fn parse_arithmetic_exp(input: Span) -> IResult<Span, AExp> {
    todo!()
}

fn parse_num(input: Span) -> IResult<Span, Statement> {
    todo!()
}

fn parse_arith_op(input: Span) -> IResult<Span, AOp> {
    todo!()
}

fn parse_bool_exp(input: Span) -> IResult<Span, BExp> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ident() {
        let ident = parse_ident(Span::new("ident")).unwrap().1;
        assert_eq!(ident, Ident::from("ident"))
    }
}


//
// pub enum AExp {
//     Var(String),
//     Num(i64),
//     BinExp(Box<AExp>, AOp, Box<AExp>)
// }
//
// pub enum AOp {
//     Plus,
//     Minus,
//     Times,
//     Fraction
// }
//
// pub enum BExp {
//     True,
//     False,
//     Not(BExp),
//     BinAExp(Box<AExp>, ROp, Box<AExp>),
//     BinBExp(Box<BExp>, BOp, Box<BExp>)
// }
//
// pub enum ROp {
//     Lt,
//     Gt,
//     Leq,
//     Geq,
//     Eq
// }
//
// pub enum BOp {
//     And,
//     Or,
//     Xor
// }


/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<I, O, E, F>(inner: F) -> impl FnMut(I) -> IResult<I, O, E>
    where
        I: InputTakeAtPosition,
        <I as InputTakeAtPosition>::Item: AsChar + Clone,
        E: ParseError<I>,
        F: Parser<I, O, E>,
{
    delimited(
        space0,
        inner,
        space0
    )
}

fn mws<I, O, E, F>(inner: F) -> impl FnMut(I) -> IResult<I, O, E>
    where
        I: InputTakeAtPosition,
        <I as InputTakeAtPosition>::Item: AsChar + Clone,
        E: ParseError<I>,
        F: Parser<I, O, E>,
{
    delimited(
        multispace0,
        inner,
        multispace0
    )
}