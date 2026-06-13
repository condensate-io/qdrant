pub const BLOCK_SIZE: usize = 32;

pub struct FastScanStorage {
    pub vector_bytes: usize,
    pub blocks: Vec<u8>,
    pub scaling_factors: Vec<f32>,
    pub l2_lengths: Vec<f32>,
}

impl FastScanStorage {
    pub fn new(vector_bytes: usize) -> Self {
        Self {
            vector_bytes,
            blocks: Vec::new(),
            scaling_factors: Vec::new(),
            l2_lengths: Vec::new(),
        }
    }

    pub fn push_block(&mut self, vectors: &[&[u8]], scalings: &[f32], l2s: &[f32]) {
        let block_offset = self.blocks.len();
        self.blocks.resize(block_offset + BLOCK_SIZE * self.vector_bytes, 0);
        for i in 0..self.vector_bytes {
            for (j, &v) in vectors.iter().enumerate() {
                self.blocks[block_offset + i * BLOCK_SIZE + j] = v[i];
            }
        }
        self.scaling_factors.extend_from_slice(scalings);
        self.l2_lengths.extend_from_slice(l2s);
        for _ in vectors.len()..BLOCK_SIZE {
            self.scaling_factors.push(0.0);
            self.l2_lengths.push(0.0);
        }
    }

    pub fn get_block(&self, block_idx: usize) -> Option<(&[u8], &[f32], &[f32])> {
        let offset = block_idx * BLOCK_SIZE * self.vector_bytes;
        let ext_offset = block_idx * BLOCK_SIZE;
        if offset + BLOCK_SIZE * self.vector_bytes <= self.blocks.len() {
            Some((
                &self.blocks[offset..offset + BLOCK_SIZE * self.vector_bytes],
                &self.scaling_factors[ext_offset..ext_offset + BLOCK_SIZE],
                &self.l2_lengths[ext_offset..ext_offset + BLOCK_SIZE],
            ))
        } else {
            None
        }
    }
}
