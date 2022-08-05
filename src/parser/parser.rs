use super::ast::*;
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};

use chumsky::prelude::*;
use chumsky::Stream;
use core::fmt;
use std::fs::*;
use std::io;
use std::sync::Arc;
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
    Add,
    Mult,
    Div,
    Bool(bool),
    Num(Arc<String>),
    Str(Arc<String>),
    Define,
    LParen,
    RParen,
    Ident(Arc<String>),
    Le,
    Leq,
    Ge,
    Geq,
    Eq,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Add => write!(f, "+"),
            Token::Mult => write!(f, "*"),
            Token::Div => write!(f, "/"),
            Token::Neg => write!(f, "-"),
            Token::Bool(b) => {
                let s = match b {
                    true => "#t",
                    false => "#f",
                };
                write!(f, "{}", s)
            }
            Token::Num(v) => write!(f, "{}", v),
            Token::Str(s) => write!(f, "\"{}\"", s),
            Token::Define => write!(f, "define"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Ident(s) => write!(f, "{}", s),
            Token::Le => write!(f, "<"),
            Token::Leq => write!(f, "<="),
            Token::Ge => write!(f, ">"),
            Token::Geq => write!(f, ">="),
            Token::Eq => write!(f, "=="),
        }
    }
}

fn lexer() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
    let neg = just('-').map(|_| Token::Neg);
    let add = just('+').map(|_| Token::Add);
    let mult = just('*').map(|_| Token::Mult);
    let div = just('/').map(|_| Token::Div);

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
    let ident = text::ident().map(|s| Token::Ident(Arc::new(s)));

    let posnum = text::int(10)
        .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
        .collect::<String>()
        .map(|s| Token::Num(Arc::new(s)));

    let str_ = just('"')
        .ignore_then(filter(|c| *c != '"').repeated())
        .then_ignore(just('"'))
        .collect::<String>()
        .map(|s| Token::Str(Arc::new(s)));

    let token = posnum
        .or(str_)
        .or(neg)
        .or(add)
        .or(mult)
        .or(div)
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

    token
        .map_with_span(|tok, span| (tok, span))
        .padded()
        .repeated()
}

fn parser() -> impl Parser<Token, Spanned<LispVal>, Error = Simple<Token>> + Clone {
    recursive(|expr| {
        let ident = select! { Token::Ident(ident) => ident.clone() }.labelled("identifier");
        let params = ident.repeated();

        let num = select! { Token::Num(v) => v.clone() }.labelled("number");
        let num = num.map_with_span(|v, span: Span| Spanned {
            x: Arc::new(LispValX::Int(v)),
            span,
        });

        let string = select! { Token::Str(s) => s.clone() }.labelled("string");
        let string = string.map_with_span(|v, span| Spanned {
            x: Arc::new(LispValX::String(v)),
            span,
        });

        let boolean = select! { Token::Bool(b) => b.clone() }.labelled("bool");
        let boolean = boolean.map_with_span(|b, span| Spanned {
            x: Arc::new(LispValX::Bool(b)),
            span,
        });

        let atom = select! { Token::Ident(ident) => ident.clone() }.labelled("atom");
        let atom = atom.map_with_span(|s, span| Spanned {
            x: Arc::new(LispValX::Atom(s)),
            span,
        });

        let arith_ops = (just(Token::Neg)
            .or(just(Token::Add))
            .or(just(Token::Div))
            .or(just(Token::Mult)))
        .map_with_span(|tok, span: Span| Spanned {
            x: Arc::new(LispValX::Atom(Arc::new(tok.to_string()))),
            span,
        })
        .then(expr.clone().repeated())
        .map_with_span(|(op, mut args), span: Span| Spanned {
            x: Arc::new(LispValX::List(Arc::new({
                let mut v = Vec::new();
                v.push(op);
                v.append(&mut args);
                v
            }))),
            span,
        });

        let lvals = just(Token::LParen)
            .ignore_then(expr.repeated())
            .then_ignore(just(Token::RParen))
            .map_with_span(|lvals, span| Spanned {
                x: Arc::new(LispValX::List(Arc::new(lvals))),
                span,
            });

        let body = lvals.clone().repeated();

        let func_name_with_params = just(Token::LParen)
            .ignore_then(just(Token::Define))
            .ignore_then(just(Token::LParen))
            .ignore_then(ident)
            .then(params)
            .then_ignore(just(Token::RParen))
            .then(body)
            .then_ignore(just(Token::RParen))
            .map_with_span(|((ident, params), body), span: Span| Spanned{
                x: Arc::new(LispValX::Function{name: ident, params, body: Arc::new(body)}), 
                span
            });

        let out = func_name_with_params
            .or(arith_ops)
            .or(num)
            .or(boolean)
            .or(string)
            .or(lvals)
            .or(atom);
        out
    })
}

fn module_parser() -> impl Parser<Token, LispModule, Error=Simple<Token>> + Clone {
    parser().repeated().then_ignore(end()).map(|fns| Arc::new(LispModuleX {functions: Arc::new(fns)}))

}

pub fn parse_file(filename: &str) -> Result<(), ParseError> {
    let src = read_to_string(filename)?;

    let len = src.chars().count();
    let (tokens, errs) = lexer().parse_recovery(src.as_str());

    let parse_errs = if let Some(tokens) = tokens {
        let len = src.chars().count();
        let (ast, parse_errs) =
            module_parser().parse_recovery(Stream::from_iter(len..len + 1, tokens.into_iter()));

        println!("{:?}", ast);
        parse_errs
    } else {
        Vec::new()
    };
    errs.into_iter()
        .map(|e| e.map(|c| c.to_string()))
        .chain(parse_errs.into_iter().map(|e| e.map(|tok| tok.to_string())))
        .for_each(|e| {
            let report = Report::build(ReportKind::Error, (), e.span().start);

            let report = match e.reason() {
                chumsky::error::SimpleReason::Unclosed { span, delimiter } => report
                    .with_message(format!(
                        "Unclosed delimiter {}",
                        delimiter.fg(Color::Yellow)
                    ))
                    .with_label(
                        Label::new(span.clone())
                            .with_message(format!(
                                "Unclosed delimiter {}",
                                delimiter.fg(Color::Yellow)
                            ))
                            .with_color(Color::Yellow),
                    )
                    .with_label(
                        Label::new(e.span())
                            .with_message(format!(
                                "Must be closed before this {}",
                                e.found()
                                    .unwrap_or(&"end of file".to_string())
                                    .fg(Color::Red)
                            ))
                            .with_color(Color::Red),
                    ),
                chumsky::error::SimpleReason::Unexpected => report
                    .with_message(format!(
                        "{}, expected {}",
                        if e.found().is_some() {
                            "Unexpected token in input"
                        } else {
                            "Unexpected end of input"
                        },
                        if e.expected().len() == 0 {
                            "something else".to_string()
                        } else {
                            e.expected()
                                .map(|expected| match expected {
                                    Some(expected) => expected.to_string(),
                                    None => "end of input".to_string(),
                                })
                                .collect::<Vec<_>>()
                                .join(", ")
                        }
                    ))
                    .with_label(
                        Label::new(e.span())
                            .with_message(format!(
                                "Unexpected token {}",
                                e.found()
                                    .unwrap_or(&"end of file".to_string())
                                    .fg(Color::Red)
                            ))
                            .with_color(Color::Red),
                    ),
                chumsky::error::SimpleReason::Custom(msg) => report.with_message(msg).with_label(
                    Label::new(e.span())
                        .with_message(format!("{}", msg.fg(Color::Red)))
                        .with_color(Color::Red),
                ),
            };

            report.finish().print(Source::from(&src)).unwrap();
        });

    todo!();
}
