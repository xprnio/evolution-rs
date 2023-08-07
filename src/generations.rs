use crate::{creatures::Creature, evolution};

#[derive(Clone)]
pub struct Generation {
    pub generation: usize,
    pub creatures: Vec<Creature>,
}

pub struct CreatureFitness<'a> {
    pub creature: &'a Creature,
    pub fitness: f32,
}

impl std::fmt::Debug for CreatureFitness<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "CreatureFitness ({}): \n{}", self.fitness, self.creature)
    }
}

impl std::fmt::Display for CreatureFitness<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Generation {
    pub fn new(n_creatures: usize, n_genes: usize) -> Generation {
        let creatures = Creature::new_vec(n_creatures, n_genes);
        Generation {
            generation: 0,
            creatures,
        }
    }

    pub fn run(&self) -> Generation {
        let mut vec: Vec<CreatureFitness> = vec![];

        for i in 0..self.creatures.len() {
            let creature = self.creatures.get(i).unwrap();
            let fit = CreatureFitness {
                creature,
                fitness: creature.fitness(),
            };
            vec.push(fit);
        }

        let average = vec
            .iter()
            .fold(-1 as f32, |average, item| match average < 0.0 {
                true => return item.creature.fitness(),
                _ => 0.0,
            });

        Generation {
            generation: self.generation + 1,
            creatures: vec
                .into_iter()
                .filter(|item| item.fitness > average)
                .map(|i| i.creature)
                .cloned()
                .collect(),
        }
    }

    pub fn kill(&mut self, kill: fn(&Creature) -> bool) {
        let mut creatures = self.creatures.clone();
        creatures.retain(|c| kill(c) == false);
        self.creatures = creatures;
    }

    fn find_partner(&self, index: usize) -> Option<&Creature> {
        for i in 0..self.creatures.len() {
            if i == index {
                continue;
            }

            let result = self.creatures.get(i).unwrap();
            return Some(result);
        }

        return None;
    }

    pub fn repopulate(&mut self, n: usize) {
        for parent_index in self.creatures.len()..n {
            let first_parent = self.creatures.get(parent_index).unwrap();
            let second_parent = self.find_partner(parent_index).unwrap();
            let child = evolution::breed(first_parent, second_parent);
            self.creatures.push(child);
        }
    }
}

impl std::fmt::Debug for Generation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "[Generation({})]", self.generation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUM_CREATURES: usize = 10;
    const NUM_GENES: usize = 100;

    #[test]
    fn it_creates_generation() {
        let generation = Generation::new(NUM_CREATURES, NUM_GENES);
        assert_eq!(generation.generation, 0);
        assert_eq!(generation.creatures.len(), NUM_CREATURES);

        for creature in generation.creatures.iter() {
            assert_eq!(creature.genes.len(), NUM_GENES);

            // Since all creatures should be new
            // the creature generation should be 0
            assert_eq!(creature.generation, 0);
        }
    }

    #[test]
    fn it_runs_generation() {
        let genesis = Generation::new(NUM_CREATURES, NUM_GENES);
        let generation = genesis.run();

        assert!(generation.creatures.len() < genesis.creatures.len());
    }

    #[test]
    fn it_kills_half() {
        let mut creatures = Vec::with_capacity(NUM_CREATURES);
        let max_index = NUM_CREATURES / 2;

        for i in 0..creatures.capacity() {
            let mut creature = Creature::new(NUM_GENES);
            if i < max_index {
                for t in 0..NUM_GENES {
                    creature.genes[t].v = true;
                }
            }

            match i < creatures.len() {
                true => creatures[i] = creature,
                false => creatures.push(creature),
            }
        }

        let mut generation = Generation {
            generation: 0,
            creatures,
        };
        generation.kill(|creature| creature.fitness() > 3.0);

        assert_eq!(generation.creatures.len(), max_index);
    }
}
