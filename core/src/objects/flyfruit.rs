use std::cell::RefCell;
use std::rc::Rc;




use crate::{structures::*, utils::*, Celeste};

use serde::{Deserialize, Serialize};

use super::fruit::check_fruit;

#[derive(Serialize, Deserialize)]
pub struct FlyFruit {
    off: f32,
    start: f32,
}
impl FlyFruit {
    pub fn init(_celeste: &mut Celeste, x: f32, y: f32) -> Object {
        Object {
            pos: Vector { x, y },
            spd: Vector { x: 0.0, y: 0.0 },
            rem: Vector { x: 0.0, y: 0.0 },
            spr: 26,
            hitbox: Rectangle {
                x: 0.0,
                y: 0.0,
                w: 8.0,
                h: 8.0,
            },
            flip: FlipState { x: false, y: false },
            collidable: false,
            solids: false,
            obj_type: ObjectType::FlyFruit(Rc::new(RefCell::new(Self { start: y, off: 0.5 }))),
            draw: Self::draw,
            update: Self::update,
            name: "FlyFruit",
        }
    }
    fn update(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::FlyFruit(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();
        if celeste.has_dashed {
            // sfx shit
            obj.spd.y = appr(obj.spd.y, -3.5, 0.25);
            if obj.spd.y < -16.0 {
                obj.destroy_self(celeste);
            }
        } else {
            this.off += 0.05;
            obj.spd.y = sin(this.off) * 0.5;
        }

        check_fruit(obj, celeste);
    }
    fn draw(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::FlyFruit(p) => p.clone(),
            _ => unreachable!(),
        };
        let this = tref.borrow_mut();
        obj.draw_sprite(celeste);
        for i in [-6, 6] {
            celeste.mem.spr(
                if celeste.has_dashed || sin(this.off) >= 0.0 {
                    45
                } else if obj.pos.y > this.start {
                    47
                } else {
                    46
                },
                obj.pos.x as i32 + i,
                obj.pos.y as i32 - 2,
                Some(FlipState {
                    x: i == -6,
                    y: false,
                }),
            )
        }
    }
}
