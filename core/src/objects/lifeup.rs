use std::{cell::RefCell, rc::Rc};

use crate::{structures::*, Celeste};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LifeUp {
    duration: f32,
    flash: f32,
}
impl LifeUp {
    pub fn init(_celeste: &mut Celeste, x: f32, y: f32) -> Object {
        Object {
            pos: Vector { x, y },
            spd: Vector { x: 0.0, y: -0.25 },
            rem: Vector { x: 0.0, y: 0.0 },
            spr: 1,
            hitbox: Rectangle {
                x: -1.0,
                y: -1.0,
                w: 10.0,
                h: 10.0,
            },
            flip: FlipState { x: false, y: false },
            collidable: true,
            solids: false,
            obj_type: ObjectType::LifeUp(Rc::new(RefCell::new(Self {
                duration: 30.0,
                flash: 0.0,
            }))),
            draw: ObjFunc(Self::draw),
            update: ObjFunc(Self::update),
            name: "LifeUp",
        }
    }
    pub fn update(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::LifeUp(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();
        this.duration -= 1.0;
        if this.duration <= 0.0 {
            obj.destroy_self(celeste);
        }
    }
    pub fn draw(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::LifeUp(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();
        this.flash += 0.5;
        celeste.mem.print(
            "1000".into(),
            obj.pos.x as i32 - 4,
            obj.pos.y as i32 - 4,
            7 + (this.flash % 2.0) as u8,
        )
    }
}
