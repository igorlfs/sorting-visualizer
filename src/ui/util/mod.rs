use rand::{distributions::Uniform, Rng};

pub fn gen_random_vector(floor: u32, ceil: u32, n: usize) -> Vec<u32> {
    let range = Uniform::new(floor, ceil);
    let values: Vec<u32> = rand::thread_rng().sample_iter(&range).take(n).collect();
    values
}
