// pub fn sign()
// use pico::Pico;
use crate::Celeste;

// pub fn tile_at(celeste: &Celeste, x: f32, y: f32) -> bool {
//     return celeste.mem.mget()
// }

pub fn min(v1: f32, v2: f32) -> f32 {
    f32::min(v1, v2)
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