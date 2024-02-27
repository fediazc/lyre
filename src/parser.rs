use crate::{lsystem::LSystem, lsystem::Symbol};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::{collections::HashMap, fs, hash::Hash};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LSysParser;

pub fn parse_file(file_name: &str) -> Result<LSystem, &'static str> {
    let unparsed_file = match fs::read_to_string(file_name) {
        Ok(s) => s,
        Err(_) => return Err("could not read file"),
    };

    let file = match LSysParser::parse(Rule::file, &unparsed_file) {
        Ok(mut pairs) => match pairs.next() {
            Some(file) => file,
            None => unreachable!(),
        },
        Err(_) => return Err("could not parse file"),
    };

    let mut productions = HashMap::<Symbol, Vec<Symbol>>::new();
    let mut axiom = Vec::<Symbol>::new();

    for rule in file.into_inner() {
        match rule.as_rule() {
            Rule::prodlist => {
                for prod in rule.into_inner() {
                    let (lhs, rhs) = parse_prod(prod)?;
                    productions.insert(lhs, rhs);
                }
            }
            Rule::axiom => {
                let list = match rule.into_inner().next() {
                    Some(s) => s,
                    None => unreachable!(),
                };

                axiom = parse_symlist(list)?;
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    if productions.is_empty() {
        for sym in &axiom {
            productions.insert(*sym, vec![*sym]);
        }
    }

    Ok(LSystem::new(productions, axiom))
}

fn parse_prod(prod: Pair<Rule>) -> Result<(Symbol, Vec<Symbol>), &'static str> {
    let mut inner_prod = prod.into_inner();

    let sym = match inner_prod.next() {
        Some(s) => s,
        None => unreachable!(),
    };

    let prod_lhs = parse_symbol(sym)?;

    let list = match inner_prod.next() {
        Some(s) => s,
        None => unreachable!(),
    };

    let prod_rhs = parse_symlist(list)?;

    Ok((prod_lhs, prod_rhs))
}

fn parse_symlist(symlist: Pair<Rule>) -> Result<Vec<Symbol>, &'static str> {
    let mut res = vec![];
    for sym in symlist.into_inner() {
        res.push(parse_symbol(sym)?);
    }
    Ok(res)
}

fn parse_symbol(symbol: Pair<Rule>) -> Result<Symbol, &'static str> {
    let symbol_char = match symbol.as_str().chars().next() {
        Some(ch) => ch,
        None => return Err("could not parse symbol"),
    };

    match symbol_char {
        '[' => Ok(Symbol::Push),
        ']' => Ok(Symbol::Pop),
        '+' => Ok(Symbol::Plus),
        '-' => Ok(Symbol::Minus),
        'S' => Ok(Symbol::Play),
        ch => Ok(Symbol::Letter(ch)),
    }
}
