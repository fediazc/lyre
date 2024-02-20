use crate::{lsystem::Element, lsystem::LSystem, note::Note};
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::{collections::HashMap, fs};

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

    let mut rule_map = HashMap::<Element, Vec<Element>>::new();
    let mut axiom: Option<Element> = None;

    for section in file.into_inner() {
        match section.as_rule() {
            Rule::section => {
                for block in section.into_inner() {
                    match block.as_rule() {
                        Rule::rule_block => {
                            for rule in block.into_inner() {
                                let lhs_element: Element;
                                let mut rhs_elements: Vec<Element> = vec![];
                                match rule.as_rule() {
                                    Rule::rule => {
                                        let mut inner_rule = rule.into_inner();

                                        let rule_lhs = match inner_rule.next() {
                                            Some(lhs) => lhs,
                                            // The None case is unreachable because there should
                                            // always be a 'rule_lhs' available at this point,
                                            // according to the grammar.
                                            None => unreachable!(),
                                        };

                                        let parsed_note = match rule_lhs.into_inner().next() {
                                            Some(note) => parse_note(note)?,
                                            None => unreachable!(),
                                        };

                                        lhs_element = Element::new(parsed_note);

                                        let rule_rhs = match inner_rule.next() {
                                            Some(rhs) => rhs,
                                            None => unreachable!(),
                                        };

                                        for note in rule_rhs.into_inner() {
                                            let parsed_note = parse_note(note)?;
                                            rhs_elements.push(Element::new(parsed_note));
                                        }
                                    }
                                    _ => unreachable!(),
                                }
                                rule_map.insert(lhs_element, rhs_elements);
                            }
                        }
                        Rule::axiom => {
                            let note = match block.into_inner().next() {
                                Some(note) => parse_note(note)?,
                                None => unreachable!(),
                            };
                            axiom = Some(Element::new(note));
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }

    let axiom = match axiom {
        Some(elem) => elem,
        None => return Err("missing axiom"),
    };

    Ok(LSystem::new(rule_map, axiom))
}

fn parse_note(note: Pair<Rule>) -> Result<Note, &'static str> {
    let mut inner_note = note.into_inner();

    let note_midi_num = {
        let note_name = match inner_note.next() {
            Some(sub) => match sub.as_rule() {
                Rule::note_name => sub,
                _ => return Err("expected note_name sub-rule while parsing note rule"),
            },
            None => unreachable!(),
        };

        let mut inner_note_name = note_name.into_inner();
        let pitch = match inner_note_name.next() {
            Some(sub) => match sub.as_rule() {
                Rule::pitch => sub.as_str(),
                _ => return Err("expected pitch sub-rule while parsing note_name rule"),
            },
            None => unreachable!(),
        };

        let octave: u8 = match inner_note_name.next() {
            Some(number) => match number.as_str().parse() {
                Ok(n) => n,
                Err(_) => return Err("could not parse octave rule"),
            },
            None => unreachable!(),
        };

        let base_midi_num = match pitch {
            "C" => 12,
            "Db" => 13,
            "D" => 14,
            "Eb" => 15,
            "E" => 16,
            "F" => 17,
            "Gb" => 18,
            "G" => 19,
            "Ab" => 20,
            "A" => 21,
            "Bb" => 22,
            "B" => 23,
            "C#" => 13,
            "D#" => 15,
            "E#" => 17,
            "F#" => 18,
            "G#" => 20,
            "A#" => 22,
            "B#" => 24,
            _ => unreachable!(),
        };

        12 * octave + base_midi_num
    };

    let (duration, velocity) = match inner_note.next() {
        Some(note_params) => {
            let mut inner_note_params = note_params.into_inner();

            let dur: u64 = match inner_note_params.next() {
                Some(number) => match number.as_str().parse() {
                    Ok(n) => n,
                    Err(_) => return Err("could not parse duration parameter"),
                },
                None => unreachable!(),
            };

            let vel: u64 = match inner_note_params.next() {
                Some(number) => match number.as_str().parse() {
                    Ok(n) => n,
                    Err(_) => return Err("could not parse velocity parameter"),
                },
                None => unreachable!(),
            };

            let vel = vel.clamp(0, 127) as u8;

            (dur, vel)
        }
        None => (4, 127),
    };

    Ok(Note::new(note_midi_num, duration, velocity))
}
