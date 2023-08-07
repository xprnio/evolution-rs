use std::cmp;

use crate::creatures::Creature;
use crate::genes::Gene;

pub fn breed(first_parent: &Creature, second_parent: &Creature) -> Creature {
    let mut genes = first_parent.genes.clone();

    for i in 0..cmp::max(first_parent.genes.len(), second_parent.genes.len()) {
        let first = first_parent.genes.get(i);
        let second = second_parent.genes.get(i);
        genes[i] = Gene {
            t: i,
            v: match (first, second) {
                (Some(first), Some(second)) => match (first.v, second.v) {
                    (true, true) => true,
                    (true, false) => false,
                    (false, true) => false,
                    (false, false) => true,
                },
                (None, Some(second)) => second.v,
                (Some(first), None) => first.v,
                (None, None) => panic!("Out of bounds"),
            },
        };
    }

    Creature {
        generation: 0,
        genes,
    }
}
