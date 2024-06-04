// Allowing dead code since this is a utilities module
#![allow(dead_code)]

pub fn round(value: f64, decimals: u32) -> f64 {
    let factor = 10_f64.powi(decimals as i32);
    (value * factor).round() / factor
}