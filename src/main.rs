use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Position {
    x: u64,
    y: u64,
}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
}

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
