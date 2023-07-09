use std::{cell::RefCell, rc::Rc};

use rand::Rng;

use crate::{structures::*, utils::sign, Celeste};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    index: f32,
    last: f32,
}
impl Message {
    pub fn init(celeste: &mut Celeste, x: f32, y: f32) -> Object {
        Object {
            pos: Vector { x, y },
            spd: Vector { x: 0.0, y: 0.0 },
            rem: Vector { x: 0.0, y: 0.0 },
            spr: 1,
            hitbox: Rectangle {
                x: 0.0,
                y: 0.0,
                w: 16.0,
                h: 16.0,
            },
            flip: FlipState { x: false, y: false },
            collidable: true,
            solids: false,
            obj_type: ObjectType::Message(Rc::new(RefCell::new(Self {
                index: 0.0,
                last: 0.0,
            }))),
            draw: Self::draw,
            update: Self::update,
            name: "FakeWall",
        }
    }
    fn update(obj: &mut Object, celeste: &mut Celeste) {}
    fn draw(obj: &mut Object, celeste: &mut Celeste) {
        let text = "-- celeste mountain --#this memorial to those# perished on the climb";
        let tref = match &mut obj.obj_type {
            ObjectType::Message(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();

        if obj.check(celeste, "Player", 4.0, 0.0).is_some() {
            if this.index < text.len() as f32 {
                this.index += 0.5;
            }
        }
    }
}
