use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Element {
    symbol: char,
}

#[derive(Debug)]
pub struct LSystem {
    alphabet: HashSet<Element>,
    rules: HashMap<Element, Vec<Element>>,
    axiom: Element,
    elements: Vec<Element>,
    step: u32,
}

impl LSystem {
    pub fn new(
        alphabet: HashSet<Element>,
        rules: HashMap<Element, Vec<Element>>,
        axiom: Element,
    ) -> Self {
        Self {
            alphabet,
            rules,
            axiom,
            elements: vec![axiom],
            step: 0,
        }
    }

    pub fn forward(&mut self, generations: u32) {
        let mut new_elements = vec![];
        for _ in 0..generations {
            new_elements.clear();
            for elem in self.elements.iter() {
                match self.rules.get(elem) {
                    Some(rhs) => new_elements.extend(rhs.iter()),
                    None => new_elements.push(*elem),
                }
            }

            self.elements.clear();
            self.elements.extend(new_elements.iter());

            self.step += 1;
        }
    }
}

impl Display for LSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s2 = format!(
            "Result: {}\nStep: {}\nAxiom: {}\nAlphabet: {}\nRules: {}",
            self.elements.iter().map(|e| e.symbol).collect::<String>(),
            self.step.to_string(),
            self.axiom.symbol,
            self.alphabet
                .iter()
                .map(|e| format!("{} ", e.symbol))
                .collect::<String>(),
            self.rules
                .iter()
                .map(|r| format!(
                    "[{} -> {}] ",
                    r.0.symbol,
                    r.1.iter().map(|e| e.symbol).collect::<String>()
                ))
                .collect::<String>()
        );

        write!(f, "{}", s2)
    }
}

impl Element {
    pub fn new(symbol: char) -> Self {
        Self { symbol }
    }
}
