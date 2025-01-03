use bevy::{prelude::*, window::PrimaryWindow};
use bevy_game::utils::random::{random_choice, uniform};
use std::time::Duration;
const PLAYER_SPEED: f32 = 200.0;
const ENEMY_SPEED: f32 = 200.0;
const N_ENEMIES: usize = 3;

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Acceleration(Vec3);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Resource)]
struct ObjectsTimer(Timer);

impl Velocity {
    fn random(scale: f32) -> Velocity {
        let velocity = uniform(-1., 1., scale);
        Velocity(velocity)
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let path = "sprites/space_shooter/Ships/spaceShips_009.png";
    let sprite_player = Sprite::from_image(asset_server.load(path));
    let transform = Transform::from_scale(Vec3::splat(0.5));
    commands.spawn((Player, sprite_player, transform));
}

fn spawn_enemies(mut commands: Commands, asset_server: Res<AssetServer>) {
    let path001 = "sprites/space_shooter/Meteors/spaceMeteors_001.png".to_string();
    let path002 = "sprites/space_shooter/Meteors/spaceMeteors_002.png".to_string();
    let path003 = "sprites/space_shooter/Meteors/spaceMeteors_003.png".to_string();
    let path004 = "sprites/space_shooter/Meteors/spaceMeteors_004.png".to_string();
    let paths = vec![path001, path002, path003, path004];
    for _ in 0..N_ENEMIES {
        let velocity = Velocity::random(ENEMY_SPEED);
        let acceleration = Acceleration(Vec3::new(0.0, 0.0, 0.0));
        let path = random_choice(&paths);
        let sprite: Sprite = Sprite::from_image(asset_server.load(path));
        let transform = Transform::from_scale(Vec3::splat(0.3));
        commands.spawn((Enemy, velocity, acceleration, sprite, transform));
    }
}
fn spawn_camera(mut commands: Commands) {
    let camera = Camera2d::default();
    commands.spawn((camera, MainCamera));
}

fn update_enemy_position(
    mut query: Query<(&mut Transform, &mut Velocity, &Acceleration), With<Enemy>>,
    time: Res<Time>,
    mut res_timer: ResMut<ObjectsTimer>,
) {
    let delta = Duration::from_secs_f32(time.delta_secs());
    let timer = &mut res_timer.0;
    let dt = timer.duration().as_secs_f32();
    if timer.tick(delta).just_finished() {
        query
            .iter_mut()
            .for_each(|(mut transformation, mut velocity, accel)| {
                velocity.0 += accel.0 * dt;
                transformation.translation += velocity.0 * dt;
            });
    }
}

fn attract_enemies_to_player(
    player_query: Query<&Transform, With<Player>>,
    mut enemies_query: Query<(&Transform, &mut Acceleration), With<Enemy>>,
) {
    let player = player_query.get_single().unwrap();
    let position = player.translation;
    enemies_query.iter_mut().for_each(|(transf, mut accel)| {
        let self_position = transf.translation;
        accel.0 = position - self_position;
    });
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
    }
}

fn mouse_aiming(
    mut query: Query<&mut Transform, With<Player>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();

    if let Some(position) = q_windows
        .single()
        .cursor_position()
        .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)))
        .map(|ray| ray.unwrap().origin.truncate())
    {
        let mut transform = query.get_single_mut().unwrap();
        let player_position = transform.translation;
        let dir = Vec3::new(position.x, position.y, 0.0) - player_position;
        let angle = dir.y.atan2(dir.x);
        let sprite_offset = -std::f32::consts::FRAC_PI_2;
        transform.rotation = Quat::from_rotation_z(angle - sprite_offset);
    } else {
        println!("Cursor is not in the game window.");
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
        .add_systems(Update, attract_enemies_to_player)
        .add_systems(Update, mouse_aiming)
        .run();
}
