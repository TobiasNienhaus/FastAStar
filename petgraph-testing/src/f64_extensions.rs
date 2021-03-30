pub fn random_f64() -> f64 {
    random_f64_inf_threshold(0.01)
}

pub fn random_f64_inf_threshold(threshold: f64) -> f64 {
    let r: f64 = rand::random();
    if r < threshold / 2.0 {
        f64::INFINITY
    } else if r < threshold {
        f64::NEG_INFINITY
    } else {
        rand::random::<f64>()
    }
}
