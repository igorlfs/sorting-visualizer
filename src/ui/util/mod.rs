use rand::{distributions::Uniform, Rng};

pub fn gen_random_vector(floor: usize, ceil: usize, n: usize) -> Vec<usize> {
    let range = Uniform::new(floor, ceil);
    rand::thread_rng().sample_iter(&range).take(n).collect()
}
