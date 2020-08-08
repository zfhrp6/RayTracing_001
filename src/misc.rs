use rand::{rngs, Rng, SeedableRng};

pub fn random_seed(seed: u8) -> rand::rngs::StdRng {
    SeedableRng::from_seed([seed; 32])
}

pub fn random(rng: &mut rngs::StdRng) -> f32 {
    rng.gen::<f32>()
}

pub fn degree_to_radian(d: f32) -> f32 {
    std::f32::consts::PI * d / 180.0
}

#[cfg(test)]
#[test]
fn random_test() {
    let rng = &mut random_seed(3u8);
    assert!(random(rng) > 0.0);
    assert!(random(rng) < 1.0);
    assert_ne!(random(rng), random(rng));
    assert_ne!(random(rng), random(rng));
    assert_ne!(random(rng), random(rng));
}
