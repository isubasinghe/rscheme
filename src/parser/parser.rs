// use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use super::ast::*;

use chumsky::Stream;
use chumsky::prelude::*;
use core::fmt;
use std::fs::*;
use std::io;
//

pub fn merge_span(s1: Span, s2: Span) -> Span {
    let (low1, high1) = (s1.start(), s1.end());
    let (low2, high2) = (s2.start(), s2.end());
    let low = std::cmp::max(low1, low2);
    let high = std::cmp::max(high1, high2);
    low..high

}
#[derive(Debug)]
pub struct ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

impl From<io::Error> for ParseError {
    fn from(_: io::Error) -> Self {
        ParseError {}
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
enum Token {
    Neg,
    Bool(bool), 
    Num(String), 
    Str(String), 
    Define,
    LParen, 
    RParen, 
    Ident(String), 
    Le, 
    Leq, 
    Ge, 
    Geq, 
    Eq
    
}

fn lexer() -> impl Parser<char, Vec<(Token, Span)>, Error=Simple<char>> {
    
    let neg = just('-').map(|_|Token::Neg);
    
    let lparen = just('(').map(|_| Token::LParen);
    let rparen = just(')').map(|_| Token::RParen);
    let le = just('<').map(|_| Token::Le);
    let leq = just("<=").map(|_| Token::Leq);
    let ge = just('>').map(|_| Token::Ge);
    let geq = just(">=").map(|_| Token::Geq);
    let eq = just("==").map(|_| Token::Eq);

    let ltrue = just("#t").map(|_| Token::Bool(true));
    let lfalse = just("#f").map(|_| Token::Bool(false));

    let define = just("define").map(|_| Token::Define);
    let ident = text::ident().map(Token::Ident);


    let posnum = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
        .collect::<String>()
        .map(Token::Num);
    
    let str_ = just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(Token::Str);

   let token = posnum
                .or(str_)
                .or(neg)
                .or(lparen)
                .or(rparen)
                .or(define)
                .or(ident)
                .or(le)
                .or(leq)
                .or(ge)
                .or(geq)
                .or(eq)
                .or(ltrue)
                .or(lfalse);

    token.map_with_span(|tok, span| (tok, span))
        .padded()
        .repeated()
}

fn parser() -> impl Parser<Token, Spanned<LispVal>, Error = Simple<Token>> + Clone {
    recursive(|expr| {

        let ident = select! { Token::Ident(ident) => ident.clone() }.labelled("identifier");
        let params = ident.repeated();

        let body_internal = expr.repeated();
        let body = just(Token::LParen)
                    .ignore_then(body_internal)
                    .then_ignore(just(Token::RParen))
                    .map(|asd| todo!());

        let func_name_with_params = just(Token::LParen)
                        .ignore_then(just(Token::Define)) 
                        .ignore_then(just(Token::LParen))
                        .ignore_then(ident)
                        .then(params)
                        .then_ignore(just(Token::RParen))
                        .then_ignore(just(Token::RParen))
                        .map(|(ident, params)| todo!());
        
        func_name_with_params
    })
}



pub fn parse_file(filename: &str) -> Result<(), ParseError> {
    let src = read_to_string(filename)?;

    let len = src.chars().count();
    let (tokens_with_spans, errs) = lexer().parse_recovery(src);
    if tokens_with_spans == None {
        return Err(ParseError{});
    }

    let tokens_with_spans = tokens_with_spans.unwrap();
    
    let (ast, parse_errs) = parser().parse_recovery(Stream::from_iter(len..len+1, tokens_with_spans.into_iter()));

    println!("{:?}", ast);
    todo!()
}

