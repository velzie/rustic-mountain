use std::{cell::RefCell, rc::Rc};

use crate::{structures::*, utils, Celeste};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Platform {
    last: f32,
    dir: f32,
}
impl Platform {
    pub fn init(celeste: &mut Celeste, x: f32, y: f32, spr: u8) -> Object {
        Object {
            pos: Vector { x: x - 4.0, y },
            spd: Vector { x: 0.0, y: 0.0 },
            rem: Vector { x: 0.0, y: 0.0 },
            spr,
            hitbox: Rectangle {
                x: 0.0,
                y: 0.0,
                w: 16.0,
                h: 8.0,
            },
            flip: FlipState { x: false, y: false },
            collidable: true,
            solids: false,
            obj_type: ObjectType::Platform(Rc::new(RefCell::new(Self {
                last: -4.0,
                dir: if spr == 11 { -1.0 } else { 1.0 },
            }))),
            draw: Self::draw,
            update: Self::update,
            name: "Platform",
        }
    }
    fn update(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::Platform(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();
        obj.spd.x = this.dir * 0.65;
        if obj.pos.x < -16.0 {
            obj.pos.x = 128.0;
        } else if obj.pos.x > 128.0 {
            obj.pos.x = -16.0;
        }

        if obj.check(celeste, "Player", 0.0, 0.0).is_none() {
            match obj.check(celeste, "Player", 0.0, -1.0) {
                Some(pind) => {
                    let playerref = celeste.objects[pind].clone();
                    let mut playerobj = playerref.borrow_mut();
                    playerobj.do_move(celeste, obj.pos.x - this.last, 0.0, 1.0);
                    // drop(&playerref);
                }
                None => (),
            }
        }
        this.last = obj.pos.x;
    }
    fn draw(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::Platform(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();
        for i in 0..2 {
            celeste.mem.spr(
                11 + i,
                obj.pos.x as i32 + (i * 8) as i32,
                obj.pos.y as i32 - 1,
                None,
            )
        }
    }
}
