#[allow(dead_code)]
pub fn assert_nearly(x: f32, expected: f32) {
    const EPSILON: f32 = 1e-5;
    let delta = (x - expected).abs();
    assert!( delta < EPSILON, "{} isn't approximately {}", x, expected);
}
