use std::{cell::RefCell, rc::Rc};

use rand::Rng;

use crate::{structures::*, Celeste};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FakeWall {}
impl FakeWall {
    pub fn init(celeste: &mut Celeste, x: f32, y: f32) -> Object {
        Object {
            pos: Vector { x, y },
            spd: Vector { x: 0.0, y: 0.0 },
            rem: Vector { x: 0.0, y: 0.0 },
            spr: 1,
            hitbox: Rectangle {
                x: -1.0,
                y: -1.0,
                w: 18.0,
                h: 18.0,
            },
            flip: FlipState { x: false, y: false },
            collidable: true,
            solids: false,
            obj_type: ObjectType::FakeWall(Rc::new(RefCell::new(Self {}))),
            draw: Self::draw,
            update: Self::update,
            name: "FakeWall",
        }
    }
    fn update(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::FakeWall(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();
    }
    fn draw(obj: &mut Object, celeste: &mut Celeste) {
        // celeste.mem.spr()
    }
}
