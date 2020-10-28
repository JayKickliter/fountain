use rand::{
    distributions::{DistIter, Uniform},
    rngs::StdRng,
    Rng, SeedableRng,
};

#[derive(Debug, Clone)]
pub enum Edges {
    /// A single (degree 1) edge.
    Edge(usize),
    /// A list of edges compressed into a tuple of (seed, degree).
    Seeded(u64, usize),
}

impl Edges {
    pub fn iter_with_range(&self, range: Uniform<usize>) -> impl Iterator<Item = usize> {
        match self {
            Edges::Edge(e) => EdgeIter::Edge(Some(*e)),
            Edges::Seeded(seed, degree) => EdgeIter::Seeded({
                let rng: StdRng = SeedableRng::seed_from_u64(*seed);
                rng.sample_iter(range).take(*degree)
            }),
        }
    }
}

enum EdgeIter {
    Edge(Option<usize>),
    Seeded(std::iter::Take<DistIter<Uniform<usize>, StdRng, usize>>),
}

impl Iterator for EdgeIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        match self {
            EdgeIter::Edge(e) => e.take(),
            EdgeIter::Seeded(rng) => rng.next(),
        }
    }
}

/// A Droplet is created by the Encoder.
#[derive(Debug)]
pub struct Droplet {
    /// The droptype can be based on seed or a list of edges.
    pub edges: Edges,
    /// The payload of the Droplet.
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct RxDroplet {
    pub edges_idx: Vec<usize>,
    pub data: Vec<u8>,
}
