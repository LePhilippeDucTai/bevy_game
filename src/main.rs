use bevy::prelude::*;
fn hello_world() {
    println!("hello world!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Carl Saggan".to_string())));
    commands.spawn((Person, Name("Alain Fill".to_string())));
}

fn print_persons(query: Query<&Name, With<Person>>) {
    query
        .iter()
        .for_each(|name| println!("Person : {}", name.0));
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    let name = query.iter_mut().find(|name| name.0 == "Alain Fill");
    if name.is_some() {
        name.unwrap().0 = "Alain Sugar".to_string();
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
#[derive(Resource)]
struct GreetTimer(Timer);
fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}
