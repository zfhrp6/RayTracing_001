pub fn random() -> f32 {
    rand::random::<f32>()
}

pub fn degree_to_radian(d: f32) -> f32 {
    std::f32::consts::PI * d / 180.0
}
