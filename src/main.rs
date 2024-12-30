use bevy::prelude::*;

#[derive(Component)]
struct Moving {
    velocity: Vec2,
    acceleration: Vec2,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

impl Moving {
    fn new() -> Moving {
        let velocity = Vec2::new(10.0, 0.0);
        let acceleration = Vec2::new(0.0, 0.0);
        Moving {
            velocity,
            acceleration,
        }
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let path = "sprites/ball_blue_small.png";
    let sprite_player = Sprite::from_image(asset_server.load(path));
    commands.spawn((Player, sprite_player));
}

fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    let path = "sprites/ball_red_small.png";
    for _ in 0..10 {
        let moving = Moving::new();
        let sprite = Sprite::from_image(asset_server.load(path));
        commands.spawn((Enemy, moving, sprite));
    }
}
fn spawn_camera(mut commands: Commands) {
    let camera = Camera2d::default();
    commands.spawn(camera);
}

fn move_enemy(mut query: Query<(&mut Transform, &mut Moving), With<Enemy>>, time: Res<Time>) {
    let response = query.iter_mut().next();
    let (mut transformation, mut player) = response.unwrap();
    let acceleration = player.acceleration;
    let dt = time.delta_secs();
    player.velocity += acceleration * dt;
    transformation.translation.x += player.velocity.x * dt;
    transformation.translation.y += player.velocity.y * dt;
}

fn move_player(
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let dt = time.delta_secs();
    let step = 10.0;
    let mut direction = Vec3::ZERO;
    let mut transform = query.get_single_mut().unwrap();
    if keys.pressed(KeyCode::KeyW) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if keys.pressed(KeyCode::KeyA) {
        direction += Vec3::new(-1.0, 0.0, 0.0);
    }
    if keys.pressed(KeyCode::KeyS) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }
    if keys.pressed(KeyCode::KeyD) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }
    if direction.length() > 0.0 {
        direction = direction.normalize();
    }
    transform.translation += direction * step * dt;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Update, move_enemy)
        .add_systems(Update, move_player)
        .run();
}
