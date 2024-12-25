use bevy::{ecs::query, prelude::*};
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

fn print_position(query: Query<&Position>) {
    (&query)
        .iter()
        .for_each(|position| println!("Position : {} {}", position.x, position.y));
}

fn hello_world() {
    println!("hello world!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Philippe LE".to_string())));
    commands.spawn((Person, Name("Zumradh SEVOUSA MARECAR".to_string())));
}

fn print_persons(query: Query<&Name, With<Person>>) {
    (&query)
        .iter()
        .for_each(|name| println!("Person : {}", name.0));
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Zumradh SEVOUSA MARECAR" {
            name.0 = "Zumradh LE".to_string();
            break; // We don't need to change any other names.
        }
    }
    (&mut query)
        .iter()
        .filter(|name| name.0 == "Zumradh SEVOUSA MARECAR")
        .for_each(|&mut name| name.0 = "Zumradh LE".to_string());
}

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(
            Update,
            (hello_world, (update_people, print_persons).chain()),
        )
        .run();
}
