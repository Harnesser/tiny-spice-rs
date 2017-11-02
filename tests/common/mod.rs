#[allow(dead_code)]
pub fn assert_nearly(x: f64, expected: f64) {
    const EPSILON: f64 = 1e-5;
    let delta = (x - expected).abs();
    assert!( delta < EPSILON, "{} isn't approximately {}", x, expected);
}
