pub fn map(val: f64, min1: f64, max1: f64, min2: f64, max2: f64) -> f64 {
    (max2 - min2) * ((val - min1) / (max1 - min1)) + min2
}