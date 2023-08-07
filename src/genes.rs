use rand::Rng;

#[derive(Clone, Copy)]
pub struct Gene {
    pub t: usize,
    pub v: bool,
}

impl Gene {
    pub fn new(t: usize) -> Gene {
        let mut rng = rand::thread_rng();
        let v: bool = rng.gen_range(0..1) == 1;

        Gene { t, v }
    }

    pub fn new_vec(n: usize) -> Vec<Gene> {
        let mut genes: Vec<Gene> = vec![];

        for i in 0..n {
            genes.push(Gene::new(i));
        }

        genes
    }

    pub fn mutate(&self, chance: f32) -> Result<Gene, &'static str> {
        if chance < 0.0 {
            return Err("Mutation Chance must not be negative");
        }
        if chance > 100.0 {
            return Err("Mutation Chance must not be above 100.0");
        }

        let mut rng = rand::thread_rng();
        let mut gene = self.clone();

        if chance >= rng.gen_range(0.0..100.00) {
            gene.v = !self.v;
        }

        Ok(gene)
    }
}

impl std::fmt::Debug for Gene {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "[Gene({}): {}]",
            self.t,
            match self.v {
                true => 1,
                false => 0,
            }
        )
    }
}

impl std::fmt::Display for Gene {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // The chance of a gene mutation happening.
    // This should be kept at 100% to force a mutation every time
    const MUTATION_CHANCE: f32 = 100.0;

    const NUM_GENES: usize = 100;

    #[test]
    fn it_generates_gene() {
        let gene = Gene::new(0);
        assert_eq!(gene.t, 0);
    }

    #[test]
    fn it_generates_gene_vec() {
        let genes = Gene::new_vec(NUM_GENES);
        assert_eq!(genes.len(), NUM_GENES);

        for i in 0..genes.len() {
            let gene = genes.get(i).unwrap();
            assert_eq!(gene.t, i);
        }
    }

    #[test]
    fn it_mutates_gene() {
        let original = Gene::new(0);
        let gene = original.mutate(MUTATION_CHANCE).unwrap();

        // The gene type should remain the same
        assert_eq!(original.t, gene.t);

        // Since the chance of a gene mutation is 100%
        // the gene value should always change
        assert_ne!(original.v, gene.v);
    }
}
