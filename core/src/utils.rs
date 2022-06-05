
// pub fn sign()
// use pico::Pico;
use crate::Celeste;

pub fn tile_at(celeste: &Celeste, x: f32, y: f32) -> bool {
    // return
    false
}

pub fn min(v1: f32, v2: f32) -> f32 {
    if v1 < v2 {
        v1
    } else {
        v2
    }
}
pub fn sign(v:f32) -> f32 {
    if v > 0f32 {
        1f32
    }else{
        0f32
    }
}
pub fn max(v1: f32, v2: f32) -> f32 {
    if v1 > v2 {
        v1
    } else {
        v2
    }
}
