use std::{cell::RefCell, rc::Rc};

use rand::Rng;

use crate::{structures::*, Celeste};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Smoke {
    spr: f32,
}
impl Smoke {
    pub fn init(celeste: &mut Celeste, x: f32, y: f32) -> Object {
        Object {
            pos: Vector {
                x: x + celeste.mem.rng.gen_range(-1.0..1.0),
                y: y + celeste.mem.rng.gen_range(-1.0..1.0),
            },
            spd: Vector {
                x: celeste.mem.rng.gen_range(0.3..0.5),
                y: -0.1,
            },
            rem: Vector { x: 0.0, y: 0.0 },
            spr: 29,
            hitbox: Rectangle {
                x: 0.0,
                y: 0.0,
                w: 0.0,
                h: 0.0,
            },
            flip: FlipState {
                x: celeste.mem.rng.gen(),
                y: celeste.mem.rng.gen(),
            },
            collidable: false,
            solids: false,
            obj_type: ObjectType::Smoke(Rc::new(RefCell::new(Self { spr: 29.0 }))),
            draw: ObjFunc(Self::draw),
            update: ObjFunc(Self::update),
            name: "Smoke",
        }
    }
    pub fn update(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::Smoke(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();
        this.spr += 0.2;
        if this.spr >= 32.0 {
            obj.destroy_self(celeste);
        }
        obj.spr = this.spr as u8;
    }
    pub fn draw(obj: &mut Object, celeste: &mut Celeste) {
        obj.draw_sprite(celeste);
    }
}
