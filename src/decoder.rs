use crate::{
    block::Block,
    droplet::{Droplet, RxDroplet},
    encoder::get_sample_from_rng_by_seed,
    types::{CatchResult, DropType},
};
use rand::distributions::Uniform;

/// Decoder for the Luby transform
pub struct Decoder {
    total_length: usize,
    blocksize: usize,
    unknown_chunks: usize,
    number_of_chunks: usize,
    cnt_received_drops: usize,
    blocks: Vec<Block>,
    data: Vec<u8>,
    dist: rand::distributions::Uniform<usize>,
}

#[derive(Debug)]
pub struct Statistics {
    pub cnt_droplets: usize,
    pub cnt_chunks: usize,
    pub overhead: f32,
    pub unknown_chunks: usize,
}

impl Decoder {
    /// Creates a new Decoder for LT codes
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate rand;
    /// # extern crate fountaincode;
    /// # fn main() {
    ///     use self::fountaincode::encoder::Encoder;
    ///     use self::fountaincode::decoder::Decoder;
    ///     use self::fountaincode::types::*;
    ///     use self::rand::{thread_rng, Rng};
    ///     use rand::distributions::Alphanumeric;
    ///
    ///     let s: String = thread_rng().sample_iter(Alphanumeric).take(1024).collect();
    ///     let msg = s.into_bytes();
    ///     let msg_len = msg.len();
    ///
    ///     let mut enc = Encoder::new(msg.clone(), 64, EncoderType::Random);
    ///     let mut dec = Decoder::new(msg_len, 64);
    ///
    ///     let mut decoded = vec![0; msg_len];
    ///
    ///     for drop in enc {
    ///         match dec.catch_to(drop, &mut decoded) {
    ///             CatchResult::Missing(stats) => {
    ///                 println!("Missing blocks {:?}", stats);
    ///             }
    ///             CatchResult::Finished(dec_len, stats) => {
    ///                 println!("Finished, stas: {:?}", stats);
    ///                 assert_eq!(msg, &decoded[..dec_len]);
    ///                 return
    ///             }
    ///         }
    ///     }
    /// # }
    /// ```
    pub fn new(len: usize, blocksize: usize) -> Decoder {
        let number_of_chunks = ((len as f32) / blocksize as f32).ceil() as usize;
        let data: Vec<u8> = vec![0; number_of_chunks * blocksize];
        let mut edges: Vec<Block> = Vec::with_capacity(number_of_chunks);
        for i in 0..number_of_chunks {
            let blk = Block::new(i, blocksize * i, false);
            edges.push(blk);
        }

        Decoder {
            total_length: len,
            number_of_chunks,
            unknown_chunks: number_of_chunks,
            cnt_received_drops: 0,
            blocks: edges,
            data,
            blocksize,
            dist: Uniform::new(0, number_of_chunks),
        }
    }

    fn process_droplet(&mut self, droplet: RxDroplet) {
        let mut drops: Vec<RxDroplet> = Vec::new();
        drops.push(droplet);
        while let Some(drop) = drops.pop() {
            let edges = drop.edges_idx.clone();
            // TODO: Maybe add shortcut for the first wave of
            // systematic codes, reduce overhead

            for ed in edges {
                // the list is edited, hence we copy first
                let block = self.blocks.get_mut(ed).unwrap();
                if block.is_known {
                    let mut b_drop = drop.clone();
                    for i in 0..self.blocksize {
                        b_drop.data[i] ^= self.data[block.begin_at + i];
                    }
                    let pos = b_drop.edges_idx.iter().position(|x| x == &ed).unwrap();
                    b_drop.edges_idx.remove(pos);
                } else {
                    block.edges.push(drop.clone());
                }
            }
            if drop.clone().edges_idx.len() == 1 {
                let first_idx = *drop.edges_idx.clone().get(0).unwrap();

                let block = self.blocks.get_mut(first_idx).unwrap();

                if !block.is_known {
                    {
                        let b_drop = &drop;
                        for i in 0..self.blocksize {
                            self.data[block.begin_at + i] = b_drop.data[i];
                        }
                    }
                    block.is_known = true;
                    self.unknown_chunks -= 1;

                    while let Some(mut edge) = block.edges.pop() {
                        let m_edge = &mut edge;

                        if m_edge.edges_idx.len() == 1 {
                            drops.push(edge);
                        } else {
                            for i in 0..self.blocksize {
                                m_edge.data[i] ^= self.data[block.begin_at + i]
                            }

                            let pos = m_edge
                                .edges_idx
                                .iter()
                                .position(|x| x == &block.idx)
                                .unwrap();
                            m_edge.edges_idx.remove(pos);
                            if m_edge.edges_idx.len() == 1 {
                                drops.push(edge.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn catch_to(&mut self, drop: Droplet, buf: &mut [u8]) -> CatchResult {
        self.cnt_received_drops += 1;
        let sample: Vec<usize> = match drop.droptype {
            DropType::Seeded(seed, degree) => {
                get_sample_from_rng_by_seed(seed, self.dist, degree).collect()
            }
            DropType::Edges(edges) => vec![edges],
        };

        let rxdrop = RxDroplet {
            edges_idx: sample,
            data: drop.data,
        };
        self.process_droplet(rxdrop);
        let stats = Statistics {
            cnt_droplets: self.cnt_received_drops,
            cnt_chunks: self.number_of_chunks,
            overhead: self.cnt_received_drops as f32 * 100.0 / self.number_of_chunks as f32,
            unknown_chunks: self.unknown_chunks,
        };

        if self.unknown_chunks == 0 {
            buf.copy_from_slice(&self.data[..self.total_length]);
            CatchResult::Finished(self.total_length, stats)
        } else {
            CatchResult::Missing(stats)
        }
    }
}
