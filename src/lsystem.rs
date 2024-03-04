use std::{collections::HashMap, fmt::Display};

use crate::note::{Note, Scale};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Symbol {
    Push,
    Pop,
    Plus,
    Minus,
    Play,
    Letter(char),
}

#[derive(Debug)]
pub struct LSystem {
    rules: HashMap<Symbol, Vec<Symbol>>,
    elements: Vec<Symbol>,
    step: u32,
}

impl LSystem {
    pub fn new(rules: HashMap<Symbol, Vec<Symbol>>, axiom: Vec<Symbol>) -> Self {
        Self {
            rules,
            elements: axiom,
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

    pub fn get_notes(&self, scale: Scale, start_key: u8) -> Vec<Note> {
        let mut key: u8 = start_key;
        let mut scale_degree = 0;
        let mut key_stack: Vec<u8> = vec![];
        let mut scale_degree_stack: Vec<i32> = vec![];
        let mut notes: Vec<Note> = vec![];
        let mut last_played_key = key;
        for symbol in &self.elements {
            match symbol {
                Symbol::Push => {
                    key_stack.push(key);
                    scale_degree_stack.push(scale_degree);
                }
                Symbol::Pop => {
                    key = match key_stack.pop() {
                        Some(k) => k,
                        None => key,
                    };

                    scale_degree = match scale_degree_stack.pop() {
                        Some(s) => s,
                        None => scale_degree,
                    };
                }
                Symbol::Plus => {
                    (key, _) = key.overflowing_add(scale.next_from(scale_degree));
                    scale_degree += 1;
                }
                Symbol::Minus => {
                    (key, _) = key.overflowing_sub(scale.prev_from(scale_degree));
                    scale_degree -= 1;
                }
                Symbol::Play => {
                    if key == last_played_key {
                        if let Some(note) = notes.last_mut() {
                            note.duration += 1;
                        } else {
                            notes.push(Note::new(key, 1, 127));
                        }
                    } else {
                        notes.push(Note::new(key, 1, 127));
                    }

                    last_played_key = key;
                }
                Symbol::Letter(_) => (),
            }
        }
        notes
    }
}

impl Display for LSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .elements
            .iter()
            .fold(String::new(), |acc, sym| acc + &sym.to_string());
        write!(f, "{}", s)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            Symbol::Push => '[',
            Symbol::Pop => ']',
            Symbol::Plus => '+',
            Symbol::Minus => '-',
            Symbol::Play => 'S',
            Symbol::Letter(c) => c,
        };
        write!(f, "{s}")
    }
}
