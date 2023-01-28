use rand::{distributions::Uniform, Rng};

use crate::ui;

pub fn gen_random_vector(floor: u32, ceil: u32, n: usize) -> Vec<u32> {
    let range = Uniform::new(floor, ceil);
    let values: Vec<u32> = rand::thread_rng().sample_iter(&range).take(n).collect();
    values
}

pub fn gen_bundle(floor: u32, ceil: u32, n: usize) -> ui::Bundle {
    let numbers = gen_random_vector(floor, ceil, n);
    let options: Vec<ui::Options> = vec![ui::Options::Default; n];
    ui::Bundle::new(numbers, options)
}
