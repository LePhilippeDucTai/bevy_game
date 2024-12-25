use bevy::prelude::*;
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
    query
        .iter()
        .for_each(|name| println!("Person : {}", name.0));
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    let name = query
        .iter_mut()
        .find(|name| name.0 == "Zumradh SEVOUSA MARECAR");
    if name.is_some() {
        name.unwrap().0 = "Zumradh LE".to_string();
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people);
        app.add_systems(
            Update,
            (hello_world, (update_people, print_persons).chain()),
        );
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
