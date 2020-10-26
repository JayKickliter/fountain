use crate::droplet::Droplet;
use petgraph::matrix_graph::DiMatrix;
use rand::distributions::Uniform;

/// Decoder for Luby Transform codes.
///
/// # Example
///
/// ```
/// use fountaincode::{
///     decoder::{CatchResult, Decoder},
///     encoder::{Encoder, EncoderType},
/// };
///
/// // For demonstration purposes, our message is just a range `u8`s.
/// let src_msg: Vec<u8> = (0..255).collect();
///
/// let mut enc = Encoder::ideal(src_msg.clone(), 64, EncoderType::Random);
/// let mut dec = Decoder::new(src_msg.len(), 64);
///
/// loop {
///     let drop = enc.drop();
///     match dec.catch(drop) {
///         CatchResult::Missing(stats) => {
///             println!("Missing blocks {:?}", stats);
///         }
///         CatchResult::Finished(reconstructed_msg, stats) => {
///             assert_eq!(src_msg, reconstructed_msg);
///             println!("Finished, stats: {:?}", stats);
///             break;
///         }
///     }
/// }
/// ```
pub struct Decoder {
    file_size: usize,
    drop_size: usize,
    n_chunks: usize,
    drops_caught: usize,
    distribution: Uniform<usize>,
    graph: DiMatrix<Node, usize>,
}

#[derive(Debug)]
pub struct Statistics {
    pub cnt_droplets: usize,
    pub cnt_chunks: usize,
    pub overhead: f32,
    pub unknown_chunks: usize,
}

#[derive(Debug)]
struct Node {
    data: Box<[u8]>,
    complete: bool,
}

impl Decoder {
    pub fn new(file_size: usize, drop_size: usize) -> Decoder {
        let n_chunks = (file_size + drop_size - 1) / drop_size;
        // let data: Vec<u8> = vec![0; number_of_chunks * blocksize];
        // let mut edges: Vec<Block> = Vec::with_capacity(number_of_chunks);
        // for i in 0..number_of_chunks {
        //     let blk = Block::new(i, Vec::new(), blocksize * i, false);
        //     edges.push(blk);
        // }

        let graph = DiMatrix::with_capacity(n_chunks);
        // for i in 0..n_chunks {
        //     graph.add_n
        // }

        Decoder {
            file_size,
            drop_size,
            n_chunks,
            drops_caught: 0,
            distribution: Uniform::new(0, n_chunks),
            graph,
        }
    }

    // fn process_droplet(&mut self, droplet: RxDroplet) {
    //     let mut drops: Vec<RxDroplet> = Vec::new();
    //     drops.push(droplet);
    //     while let Some(drop) = drops.pop() {
    //         let edges = drop.edges_idx.clone();
    //         // TODO: Maybe add shortcut for the first wave of
    //         // systematic codes, reduce overhead

    //         for ed in edges {
    //             // the list is edited, hence we copy first
    //             let block = self.blocks.get_mut(ed).unwrap();
    //             if block.is_known {
    //                 let mut b_drop = drop.clone();
    //                 xor_bytes(
    //                     &mut b_drop.data[..self.blocksize],
    //                     &self.data[block.begin_at..],
    //                 );
    //                 let pos = b_drop.edges_idx.iter().position(|x| x == &ed).unwrap();
    //                 b_drop.edges_idx.remove(pos);
    //             } else {
    //                 block.edges.push(drop.clone());
    //             }
    //         }
    //         if drop.clone().edges_idx.len() == 1 {
    //             let first_idx = *drop.edges_idx.clone().get(0).unwrap();

    //             let block = self.blocks.get_mut(first_idx).unwrap();

    //             if !block.is_known {
    //                 {
    //                     let b_drop = &drop;
    //                     for i in 0..self.blocksize {
    //                         self.data[block.begin_at + i] = b_drop.data[i];
    //                     }
    //                 }
    //                 block.is_known = true;
    //                 self.unknown_chunks -= 1;

    //                 while let Some(mut edge) = block.edges.pop() {
    //                     let m_edge = &mut edge;

    //                     if m_edge.edges_idx.len() == 1 {
    //                         drops.push(edge);
    //                     } else {
    //                         xor_bytes(
    //                             &mut m_edge.data[..self.blocksize],
    //                             &self.data[block.begin_at..],
    //                         );

    //                         let pos = m_edge
    //                             .edges_idx
    //                             .iter()
    //                             .position(|x| x == &block.idx)
    //                             .unwrap();
    //                         m_edge.edges_idx.remove(pos);
    //                         if m_edge.edges_idx.len() == 1 {
    //                             drops.push(edge.clone());
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    /// Catches a Droplet
    /// When it is possible to reconstruct a set, the bytes are returned
    pub fn catch(&mut self, drop: Droplet) -> CatchResult {
        self.drops_caught += 1;
        // self.cnt_received_drops += 1;
        // let sample: Vec<usize> = drop.edges.iter_with_range(self.dist).collect();
        // let rxdrop = RxDroplet {
        //     edges_idx: sample,
        //     data: drop.data,
        // };
        // self.process_droplet(rxdrop);
        // let stats = Statistics {
        //     cnt_droplets: self.cnt_received_drops,
        //     cnt_chunks: self.number_of_chunks,
        //     overhead: self.cnt_received_drops as f32 * 100.0 / self.number_of_chunks as f32,
        //     unknown_chunks: self.unknown_chunks,
        // };

        // if self.unknown_chunks == 0 {
        //     let mut result = Vec::with_capacity(self.total_length);
        //     for i in 0..self.total_length {
        //         // TODO: we should be able to do that without copying
        //         result.push(self.data[i]);
        //     }
        //     CatchResult::Finished(result, stats)
        // } else {
        //     CatchResult::Missing(stats)
        // }
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum CatchResult {
    Finished(Vec<u8>, Statistics),
    Missing(Statistics),
}
