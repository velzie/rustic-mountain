use std::f32::consts::PI;

// pub fn sign()
// use pico::Pico;


// pub fn tile_at(celeste: &Celeste, x: f32, y: f32) -> bool {
//     return celeste.mem.mget()
// }

pub fn min(v1: f32, v2: f32) -> f32 {
    f32::min(v1, v2)
}
pub fn sin(percentage: f32) -> f32 {
    // p8's trig is weird asf
    f32::sin(percentage * -2.0 * PI)
}
pub fn cos(percentage: f32) -> f32 {
    // p8's trig is weird asf
    f32::cos(percentage * -2.0 * PI)
}
pub fn sign(v: f32) -> f32 {
    if v != 0f32 {
        v.signum()
    } else {
        0f32
    }
}
pub fn max(v1: f32, v2: f32) -> f32 {
    f32::max(v1, v2)
}
pub fn appr(val: f32, target: f32, amount: f32) -> f32 {
    if val > target {
        max(val - amount, target)
    } else {
        min(val + amount, target)
    }
}
pub fn mid(v1: f32, v2: f32, v3: f32) -> f32 {
    return v1.max(v2).min(v3);
}
