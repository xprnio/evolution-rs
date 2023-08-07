use crate::genes::Gene;

#[derive(Clone)]
pub struct Creature {
    pub genes: Vec<Gene>,
    pub generation: usize,
}

impl std::fmt::Debug for Creature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(
            f,
            "Creature: \n  generation: {}\n  genes: {}",
            self.generation,
            self.genes.iter().fold("".to_owned(), |result, gene| {
                let v = match gene.v {
                    true => "1",
                    false => "0",
                };
                result + v
            }),
        )
    }
}

impl std::fmt::Display for Creature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Creature {
    pub fn new(n_genes: usize) -> Creature {
        Creature {
            generation: 0,
            genes: Gene::new_vec(n_genes),
        }
    }

    pub fn new_vec(n: usize, n_genes: usize) -> Vec<Creature> {
        let mut creatures: Vec<Creature> = vec![];

        for _ in 0..n {
            creatures.push(Creature::new(n_genes));
        }

        creatures
    }

    // TODO: Fitness should be based on Evolution
    // TODO: Fitness should be weighed by gene
    pub fn fitness(&self) -> f32 {
        let genes = self.genes.iter();
        genes.fold(0.0, |fitness, gene| match gene.v {
            true => fitness + 1.0,
            _ => fitness,
        })
    }

    pub fn evolve(&mut self, chance: f32) -> &Creature {
        for i in 0..self.genes.len() {
            let original = self.genes.get(i).unwrap();
            let gene = original.mutate(chance).unwrap();
            self.genes[i] = gene;
        }
        self.generation += 1;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::creatures::Creature;

    // The chance of a gene mutation happening.
    // This should be kept at 100% to force a mutation every time
    const MUTATION_CHANCE: f32 = 100.0;

    const NUM_CREATURES: usize = 10;
    const NUM_GENES: usize = 100;

    #[test]
    fn it_generates_creature() {
        let creature = Creature::new(NUM_GENES);
        assert_eq!(creature.genes.len(), NUM_GENES);
    }

    #[test]
    fn it_generates_creature_vec() {
        let creatures = Creature::new_vec(NUM_CREATURES, NUM_GENES);
        assert_eq!(creatures.len(), NUM_CREATURES);
        for creature in creatures.iter() {
            assert_eq!(creature.genes.len(), NUM_GENES);
        }
    }

    #[test]
    fn it_evolves_creature() {
        let original = Creature::new(NUM_GENES);
        let mut creature = original.clone();
        creature.evolve(MUTATION_CHANCE);

        assert_eq!(creature.generation, original.generation + 1);
        assert_eq!(creature.genes.len(), original.genes.len());

        for i in 0..original.genes.len() {
            let gene = original.genes.get(i).unwrap();
            let target = creature.genes.get(i).unwrap();

            // The gene type should remain the same
            assert_eq!(gene.t, target.t);

            // Since the chance of a gene mutation is 100%
            // the gene value should always change
            assert_ne!(gene.v, target.v);
        }
    }
}
