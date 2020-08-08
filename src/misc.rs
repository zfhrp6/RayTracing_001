pub fn random() -> f32 {
    rand::random::<f32>()
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
