mod lsystem;

use lsystem::{Element, LSystem};
use std::collections::{HashMap, HashSet};

fn main() {
    let elem_0 = Element::new('0');
    let elem_1 = Element::new('1');
    let elem_open = Element::new('[');
    let elem_close = Element::new(']');
    let axiom = elem_0;

    let alphabet = {
        let mut set = HashSet::new();
        set.insert(elem_0);
        set.insert(elem_1);
        set.insert(elem_close);
        set.insert(elem_open);

        set
    };

    let rules = {
        let mut map = HashMap::<Element, Vec<Element>>::new();
        map.insert(elem_0, vec![elem_1, elem_open, elem_0, elem_close, elem_0]);
        map.insert(elem_1, vec![elem_1, elem_1]);

        map
    };

    let mut system = LSystem::new(alphabet, rules, axiom);

    system.forward(4);

    println!("{system}");
}
