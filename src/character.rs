use bevy::prelude::*;

#[derive(Component)]
pub struct Character {
    pub tile_location_x: usize, 
    pub tile_location_z: usize,
}