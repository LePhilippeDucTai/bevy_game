use bevy::math::Vec3;
use rand::{distributions::Uniform, Rng};

pub fn uniform(a: f32, b: f32, scale: f32) -> Vec3 {
    let d = rand::thread_rng().sample_iter(Uniform::new(a, b));
    let u: Vec<f32> = d.take(3).collect();
    scale * Vec3::new(u[0], u[1], u[2])
}

pub fn random_integer(low: u32, high: u32) -> u32 {
    let mut gen = rand::thread_rng();
    gen.sample(Uniform::new(low, high))
}

pub fn random_choice(s: &Vec<String>) -> &String {
    let size = s.len();
    let i = random_integer(0, size as u32) as usize;
    &s[i]
}
