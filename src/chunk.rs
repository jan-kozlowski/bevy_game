use bevy::prelude::*;

pub const CHUNK_SIZE: usize = 8;

pub struct Chunk {

   pub blocks: [[u8; CHUNK_SIZE]; CHUNK_SIZE],
   location: (u8, u8, u8)
}

impl Default for Chunk {

    fn default() -> Self {
        
        Self {
            blocks: [
                [255, 0, 0, 0, 0, 0, 0, 0],
                [129, 0, 0, 0, 0, 0, 0, 0],
                [129, 0, 0, 0, 0, 0, 0, 0],
                [129, 0, 0, 0, 0, 0, 0, 0],
                [129, 0, 0, 0, 0, 0, 0, 0],
                [129, 0, 0, 0, 0, 0, 0, 0],
                [129, 0, 0, 0, 0, 0, 0, 0],
                [255, 0, 0, 0, 0, 0, 0, 0],
            ],
            location: (0, 0, 0)
        }
    }
}