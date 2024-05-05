use std::sync::Mutex;
use lazy_static::lazy_static;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

lazy_static!{
    static ref RNG: Mutex<StdRng> = Mutex::new(StdRng::seed_from_u64(1234));
}

pub fn random() -> f32 {
    let mut rng = RNG.lock().unwrap();
    rng.gen()
}

pub fn degree_to_radian(d: f32) -> f32 {
    std::f32::consts::PI * d / 180.0
}

#[cfg(test)]
#[test]
fn random_test() {
    assert!(random() > 0.0);
    assert!(random() < 1.0);
    assert_ne!(random(), random());
    assert_ne!(random(), random());
    assert_ne!(random(), random());
}
