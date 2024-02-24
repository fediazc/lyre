use std::{collections::HashMap, fmt::Display};

use crate::note::Note;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Element {
    pub note: Note,
}

#[derive(Debug)]
pub struct LSystem {
    rules: HashMap<Element, Vec<Element>>,
    axiom: Vec<Element>,
    pub elements: Vec<Element>,
    step: u32,
}

impl LSystem {
    pub fn new(rules: HashMap<Element, Vec<Element>>, axiom: Vec<Element>) -> Self {
        let mut elems = vec![];
        elems.extend(axiom.iter());

        Self {
            rules,
            axiom,
            elements: elems,
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
        let s = format!(
            "Result: {}\nStep: {}\nAxiom: {}\nRules: {}",
            self.elements
                .iter()
                .map(|e| format!("{} ", e.note.get_name()))
                .collect::<String>(),
            self.step.to_string(),
            self.axiom
                .iter()
                .map(|e| format!("{} ", e.note.get_name()))
                .collect::<String>(),
            self.rules
                .iter()
                .map(|r| format!(
                    "[{} -> {}] ",
                    r.0.note.get_name(),
                    r.1.iter()
                        .map(|e| format!("{} ", e.note.get_name()))
                        .collect::<String>()
                ))
                .collect::<String>()
        );

        write!(f, "{}", s)
    }
}

impl Element {
    pub fn new(note: Note) -> Self {
        Self { note }
    }
}
