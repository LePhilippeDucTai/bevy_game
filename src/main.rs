use bevy::prelude::*;
use bevy_game::utils::random::uniform;
use std::time::Duration;
const PLAYER_SPEED: f32 = 40.0;

#[derive(Component)]
struct Moving {
    velocity: Vec3,
    acceleration: Vec3,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Resource)]
struct ObjectsTimer(Timer);

impl Moving {
    fn random() -> Moving {
        let velocity = uniform(40.0);
        let acceleration = Vec3::ZERO;
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
    // let window = window_query.get_single().unwrap();
    for _ in 0..10 {
        let moving = Moving::random();
        let sprite = Sprite::from_image(asset_server.load(path));
        commands.spawn((Enemy, moving, sprite));
    }
}
fn spawn_camera(mut commands: Commands) {
    let camera = Camera2d::default();
    commands.spawn(camera);
}

fn update_enemy_position(
    mut query: Query<(&mut Transform, &mut Moving), With<Enemy>>,
    time: Res<Time>,
    mut res_timer: ResMut<ObjectsTimer>,
) {
    let delta = Duration::from_secs_f32(time.delta_secs());
    let timer = &mut res_timer.0;
    let dt = timer.duration().as_secs_f32();
    if timer.tick(delta).just_finished() {
        query
            .iter_mut()
            .for_each(|(mut transformation, mut player)| {
                let acceleration = player.acceleration;
                player.velocity += acceleration * dt;
                transformation.translation += player.velocity * dt;
            });
    }
}

fn control_player(
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut res_timer: ResMut<ObjectsTimer>,
) {
    let delta = Duration::from_secs_f32(time.delta_secs());
    let timer = &mut res_timer.0;
    let dt = timer.duration().as_secs_f32();

    if timer.tick(delta).just_finished() {
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
        transform.translation += direction * PLAYER_SPEED * dt;
        let test = transform.translation;
        println!("{test:?}");
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_enemies)
        .insert_resource(ObjectsTimer(Timer::from_seconds(
            0.01,
            TimerMode::Repeating,
        )))
        .add_systems(Update, (control_player, update_enemy_position))
        .run();
}
