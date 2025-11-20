use bevy::prelude::*;

pub const ROOM_SIZE_X: usize = 8;
pub const ROOM_SIZE_Z: usize = 8;

pub struct Tilemap {

   pub tiles: [[u8; ROOM_SIZE_X]; ROOM_SIZE_Z]
}

impl Default for Tilemap {

    fn default() -> Self {
        
        Self {
            tiles: [
                [255, 0, 0, 0, 0, 0, 0, 255],
                [255, 0, 0, 0, 0, 0, 0, 255],
                [255, 0, 0, 0, 0, 0, 0, 255],
                [255, 0, 0, 0, 0, 0, 0, 255],
                [255, 0, 0, 0, 0, 0, 0, 255],
                [255, 0, 0, 0, 0, 0, 0, 255],
                [255, 0, 0, 0, 0, 0, 0, 255],
                [255, 0, 0, 0, 0, 0, 0, 255],
            ]
        }
    }
}

impl Tilemap {

    pub fn get_world_location(x: usize, z: usize) -> Vec3 {

        vec3(
            x as f32, 0., z as f32
        )
    }
}