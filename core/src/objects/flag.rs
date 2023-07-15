use std::{cell::RefCell, rc::Rc};

use crate::{draw_time, structures::*, Celeste};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Flag {
    score: u8,
    show: bool,
}
impl Flag {
    pub fn init(celeste: &mut Celeste, x: f32, y: f32) -> Object {
        Object {
            pos: Vector { x: x + 5.0, y },
            spd: Vector { x: 0.0, y: 0.0 },
            rem: Vector { x: 0.0, y: 0.0 },
            spr: 1,
            hitbox: Rectangle {
                x: 0.0,
                y: 0.0,
                w: 8.0,
                h: 8.0,
            },
            flip: FlipState { x: false, y: false },
            collidable: true,
            solids: false,
            obj_type: ObjectType::Flag(Rc::new(RefCell::new(Self {
                score: celeste
                    .got_fruit
                    .iter()
                    .map(|x| if *x { 1 } else { 0 })
                    .sum(),
                // .fold(0, |acc, x| acc +),
                show: false,
            }))), // score =
            draw: ObjFunc(Self::draw),
            update: ObjFunc(Self::update),
            name: "Flag",
        }
    }
    pub fn update(_obj: &mut Object, _celeste: &mut Celeste) {}
    pub fn draw(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::Flag(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();

        obj.spr = 118 + ((celeste.frames / 5) % 3) as u8;
        obj.draw_sprite(celeste);

        if this.show {
            celeste.mem.rectfill(32, 2, 96, 31, 0);
            celeste.mem.spr(26, 55, 6, None);
            celeste.mem.print(&format!("x{}", this.score), 64, 9, 7);
            draw_time(celeste, 49, 16);
            celeste
                .mem
                .print(&format!("deaths:{}", celeste.deaths), 48, 24, 7);
        } else if obj.check(celeste, "Player", 0.0, 0.0).is_some() {
            // sfx 55
            // sfx timer = 30
            this.show = true;
        }
    }
}
