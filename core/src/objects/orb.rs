use std::{cell::RefCell, rc::Rc};

use crate::{structures::*, Celeste};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Orb {
    spr: f32,
}
impl Orb {
    pub fn init(celeste: &mut Celeste, x: f32, y: f32) -> Object {
        Object {
            pos: Vector { x, y },
            spd: Vector { x: 0.0, y: 0.0 },
            rem: Vector { x: 0.0, y: 0.0 },
            spr: 29,
            hitbox: Rectangle {
                x: 0.0,
                y: 0.0,
                w: 0.0,
                h: 0.0,
            },
            flip: FlipState { x: false, y: false },
            collidable: false,
            solids: false,
            obj_type: ObjectType::Orb(Rc::new(RefCell::new(Self { spr: 29.0 }))),
            draw: Self::draw,
            update: Self::update,
            name: "Orb",
        }
    }
    fn update(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::Orb(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();
    }
    fn draw(obj: &mut Object, celeste: &mut Celeste) {
        obj.draw_sprite(celeste);
    }
}
