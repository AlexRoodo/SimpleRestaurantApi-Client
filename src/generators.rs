use rand::{distributions::Alphanumeric, Rng};

pub fn random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

pub fn random_numb(min: i32, max: i32) -> i32 {
    rand::thread_rng().gen_range(min..=max)
}
