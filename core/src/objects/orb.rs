use std::{cell::RefCell, rc::Rc};

use crate::{
    structures::*,
    utils::{appr, cos, sin},
    Celeste,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Orb {}
impl Orb {
    pub fn init(_celeste: &mut Celeste, x: f32, y: f32) -> Object {
        Object {
            pos: Vector { x, y },
            spd: Vector { x: 0.0, y: -4.0 },
            rem: Vector { x: 0.0, y: 0.0 },
            spr: 102,
            hitbox: Rectangle {
                x: 0.0,
                y: 0.0,
                w: 8.0,
                h: 8.0,
            },
            flip: FlipState { x: false, y: false },
            collidable: false,
            solids: false,
            obj_type: ObjectType::Orb(Rc::new(RefCell::new(Self {}))),
            draw: ObjFunc(Self::draw),
            update: ObjFunc(Self::update),
            name: "Orb",
        }
    }
    pub fn update(obj: &mut Object, _celeste: &mut Celeste) {}
    pub fn draw(obj: &mut Object, celeste: &mut Celeste) {
        obj.spd.y = appr(obj.spd.y, 0.0, 0.5);
        if obj.spd.y == 0.0 {
            let hit = obj.check(celeste, "Player", 0.0, 0.0);
            match hit {
                Some(i) => {
                    let jref = celeste.objects[i].clone();
                    let mut playerobj = jref.borrow_mut();
                    let pref = match &mut playerobj.obj_type {
                        ObjectType::Player(p) => p.clone(),
                        _ => unreachable!(),
                    };

                    let mut player = pref.borrow_mut();
                    // music timer 45
                    // sfx 51
                    celeste.freeze = 10;
                    celeste.shake = 10;
                    player.djump = 2;
                    drop(player);
                    drop(playerobj);
                    drop(pref);
                    drop(jref);
                    // ^ take out of scope early
                    obj.destroy_self(celeste);
                    celeste.max_djump = 2;
                }
                _ => (),
            }
        }

        obj.draw_sprite(celeste);
        for x in 0..8 {
            let i = x as f32 * 0.125;
            celeste.mem.circfill(
                (obj.pos.x + 4.0 + cos(celeste.frames as f32 / 30.0 + i) * 8.0) as u8,
                (obj.pos.y + 4.0 + sin(celeste.frames as f32 / 30.0 + i) * 8.0) as u8,
                1,
                7,
            )
        }
    }
}
