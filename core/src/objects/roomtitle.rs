use std::{cell::RefCell, rc::Rc};

use crate::{draw_time, structures::*, Celeste};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RoomTitle {
    delay: i32,
}
impl RoomTitle {
    pub fn init(celeste: &mut Celeste, x: f32, y: f32) -> Object {
        Object {
            pos: Vector { x, y },
            spd: Vector { x: 0.0, y: 0.0 },
            rem: Vector { x: 0.0, y: 0.0 },
            spr: 0,
            hitbox: Rectangle {
                x: 0.0,
                y: 0.0,
                w: 0.0,
                h: 0.0,
            },
            flip: FlipState { x: false, y: false },
            collidable: false,
            solids: false,
            obj_type: ObjectType::RoomTitle(Rc::new(RefCell::new(Self { delay: 5 }))),
            draw: Self::draw,
            update: Self::update,
            name: "Orb",
        }
    }
    fn update(obj: &mut Object, celeste: &mut Celeste) {}
    fn draw(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::RoomTitle(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();
        this.delay -= 1;
        if this.delay < -30 {
            obj.destroy_self(celeste);
        } else if this.delay < 0 {
            celeste.mem.rectfill(24, 58, 104, 70, 0);
            if celeste.level == 11 {
                celeste.mem.print("old site", 48, 62, 7);
            } else if celeste.level == 30 {
                celeste.mem.print("summit", 52, 62, 7);
            } else {
                celeste.mem.print(
                    &format!("{}00 m", celeste.level + 1),
                    if celeste.level < 10 { 54 } else { 52 },
                    62,
                    7,
                );
            }
        }

        draw_time(celeste, 4, 4);
    }
}
