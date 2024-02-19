mod lsystem;
mod note;

use lsystem::{Element, LSystem};
use note::Note;
use std::collections::{HashMap, HashSet};

fn main() {
    let elem1 = Element::new(Note::new(69, 4, 127));
    let elem2 = Element::new(Note::new(72, 4, 127));
    let elem3 = Element::new(Note::new(74, 4, 127));
    let elem4 = Element::new(Note::new(76, 4, 127));
    let elem5 = Element::new(Note::new(79, 4, 127));
    let axiom = elem1;

    let alphabet = {
        let mut set = HashSet::new();
        set.insert(elem1);
        set.insert(elem2);
        set.insert(elem3);
        set.insert(elem4);
        set.insert(elem5);

        set
    };

    let rules = {
        let mut map = HashMap::<Element, Vec<Element>>::new();
        map.insert(elem1, vec![elem1, elem2, elem4, elem3]);
        map.insert(elem3, vec![elem3, elem5, elem1, elem2]);

        map
    };

    let mut system = LSystem::new(alphabet, rules, axiom);

    system.forward(3);

    println!("{system}");
}
