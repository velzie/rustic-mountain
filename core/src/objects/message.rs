use std::{cell::RefCell, rc::Rc};

use crate::{structures::*, Celeste};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    index: f32,
    last: f32,
}
impl Message {
    pub fn init(_celeste: &mut Celeste, x: f32, y: f32) -> Object {
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
            draw: ObjFunc(Self::draw),
            update: ObjFunc(Self::update),
            name: "Message",
        }
    }
    pub fn update(_obj: &mut Object, _celeste: &mut Celeste) {}
    pub fn draw(obj: &mut Object, celeste: &mut Celeste) {
        let text: Box<[char]> =
            "-- celeste mountain --#this memorial to those# perished on the climb"
                .chars()
                .collect();
        let tref = match &mut obj.obj_type {
            ObjectType::Message(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();

        if obj.check(celeste, "Player", 4.0, 0.0).is_some() {
            if this.index < text.len() as f32 {
                this.index += 0.5;
                if this.index >= this.last + 1.0 {
                    this.last += 1.0;
                    // sfx 35
                }
            }
            let mut _x = 8;
            let mut _y = 96;
            for i in 1..this.index as i32 {
                if text[i as usize] != '#' {
                    celeste.mem.rectfill(_x - 2, _y - 2, _x + 7, _y + 6, 7);
                    celeste.mem.print(&text[i as usize].to_string(), _x, _y, 0);
                    _x += 5;
                } else {
                    _x = 8;
                    _y += 7;
                }
            }
        } else {
            this.index = 0.0;
            this.last = 0.0;
        }
    }
}
