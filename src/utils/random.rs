use bevy::math::Vec3;
use rand::{distributions::Uniform, Rng};

pub fn uniform(scale: f32) -> Vec3 {
    let d = rand::thread_rng().sample_iter(Uniform::new(-1.0, 1.0));
    let u: Vec<f32> = d.take(3).collect();
    scale * Vec3::new(u[0], u[1], u[2])
}
