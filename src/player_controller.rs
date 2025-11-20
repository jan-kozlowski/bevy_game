use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use crate::{character::Character, tilemap::Tilemap};

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {

    fn build(&self, app: &mut App) {

        app
            .add_input_context::<PlayerController>()
            .add_observer(apply_movement);

        println!("Camera plugin built successfully!");
    }
}

#[derive(Component)]
pub struct PlayerController;

#[derive(InputAction)]
#[action_output(bool)]
pub struct MoveUp;

fn apply_movement(_input: On<Start<MoveUp>>, characters: Query<(&mut Character, &mut Transform)>) {

    info!("currently being fired!");
    for (mut character, mut transform) in characters {

        character.tile_location_z += 1;
        transform.translation = Tilemap::get_world_location(character.tile_location_x, character.tile_location_z);
    }
}