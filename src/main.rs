use std::f32::consts::PI;

use bevy::{light::CascadeShadowConfigBuilder, pbr::wireframe::Wireframe, prelude::*};
use bevy_dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig};

mod camera;
mod character;
mod player_controller;
mod tilemap;
use bevy_enhanced_input::{EnhancedInputPlugin, action::Action, actions, bindings};
use tilemap::*;

use crate::character::Character;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EnhancedInputPlugin,
            FpsOverlayPlugin {
                config: FpsOverlayConfig {
                    frame_time_graph_config: FrameTimeGraphConfig::target_fps(180.),
                    ..default()
                },
            },
        ))
        .add_plugins(
            (
                // camera::CameraPlugin,
                player_controller::PlayerControllerPlugin
            ),
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .build(),
    ));

    let tilemap = Tilemap::default();

    for x in 0..ROOM_SIZE_X {
        for z in 0..ROOM_SIZE_Z {
            let color = Color::linear_rgb(
                tilemap.tiles[z][x] as f32 / 255.,
                x as f32 / ROOM_SIZE_X as f32,
                z as f32 / ROOM_SIZE_Z as f32,
            );

            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(1.0, 0.5, 1.0))),
                MeshMaterial3d(materials.add(color)),
                Transform::from_translation(Tilemap::get_world_location(x, z)),
                Wireframe,
            ));
        }
    }

    commands.spawn((
        player_controller::PlayerController,
        actions!(
            player_controller::PlayerController[(
                Action::<player_controller::MoveUp>::new(),
                bindings![KeyCode::KeyQ]
            )]
        ),
    ));

    let start_loc = Tilemap::get_world_location(ROOM_SIZE_X / 2, 0);

    commands.spawn((
        Character {
            tile_location_x: ROOM_SIZE_X / 2,
            tile_location_z: 0,
        },
        Mesh3d(meshes.add(Cuboid::new(0.5, 2., 0.5))),
        MeshMaterial3d(materials.add(Color::BLACK)),
        Transform::from_translation(start_loc),
        Wireframe,
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(start_loc + Vec3::new(0., 5.0, -5.))
            .looking_at(start_loc, Vec3::Y),
    ));
}
