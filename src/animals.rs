use bevy::prelude::*;

#[derive(Debug)]
enum AnimalType {
    Bear,
    Dog,
    Cat,
}

pub struct AnimalPlugin;

impl Plugin for AnimalPlugin {

    fn build(&self, app: &mut App) {

        app
            .add_systems(Startup, create_animals)
            .add_systems(Update, introduce_animals)
            .insert_resource(PrintTimer{
                timer: Timer::from_seconds(2.0, TimerMode::Repeating)
            });
        println!("Plugin build success!");
    }
}

#[derive(Resource)]
struct PrintTimer {

    timer: Timer
}

#[derive(Component)]
struct Wild;

#[derive(Component)]
struct Animal {

    animal_type: AnimalType,
}

fn create_animals(mut commands: Commands) {
    
    commands.spawn((
        Animal { animal_type: AnimalType::Bear}, Wild));
    commands.spawn((
        Animal { animal_type: AnimalType::Dog}, Wild));
    commands.spawn((
        Animal { animal_type: AnimalType::Cat}, Wild));
    commands.spawn(
        Animal { animal_type: AnimalType::Dog});
}

fn introduce_animals(time: Res<Time>, mut timer: ResMut<PrintTimer>, animals_query: Query<&Animal>, wild_animals_query: Query<&Animal, With<Wild>>) {

    if !timer.timer.tick(time.delta()).just_finished() {

        return;
    }

    println!("All animals:");

    for animal in &animals_query {

        println!("I am {:?}", animal.animal_type);
    }

    println!("Wild animals:");

    for animal in &wild_animals_query {

        println!("I am {:?}", animal.animal_type);
    }
}