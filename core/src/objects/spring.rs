use std::cell::RefCell;
use std::rc::Rc;

use crate::{structures::*, Celeste};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Spring {
    pub hide_in: u8,
    hide_for: u8,
    delay: u8,
}
impl Spring {
    pub fn init(_celeste: &mut Celeste, x: f32, y: f32) -> Object {
        Object {
            pos: Vector { x, y },
            spd: Vector { x: 0.0, y: 0.0 },
            rem: Vector { x: 0.0, y: 0.0 },
            spr: 18,
            hitbox: Rectangle {
                x: 0.0,
                y: 0.0,
                w: 8.0,
                h: 8.0,
            },
            flip: FlipState { x: false, y: false },
            solids: true,
            collidable: true,
            obj_type: ObjectType::Spring(Rc::new(RefCell::new(Self {
                hide_in: 0,
                hide_for: 0,
                delay: 0,
            }))),
            draw: ObjFunc(Self::draw),
            update: ObjFunc(Self::update),
            name: "Spring",
        }
    }
    pub fn update(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::Spring(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();

        if this.hide_for > 0 {
            this.hide_for -= 1;
            if this.hide_for <= 0 {
                obj.spr = 18;
                this.delay = 0;
            }
        } else if obj.spr == 18 {
            let hit = obj.check(celeste, "Player", 0.0, 0.0);
            // dbg!(&hit);
            match hit {
                Some(i) => {
                    // panic!();'
                    let jref = celeste.objects[i].clone();
                    let mut playerobj = jref.borrow_mut();
                    let pref = match &mut playerobj.obj_type {
                        ObjectType::Player(p) => p.clone(),
                        _ => unreachable!(),
                    };

                    let mut player = pref.borrow_mut();
                    if playerobj.spd.y >= 0.0 {
                        obj.spr = 19;
                        playerobj.pos.y = obj.pos.y - 4.0;
                        playerobj.spd.x *= 0.2;
                        playerobj.spd.y = -3.0;
                        player.djump = celeste.max_djump;
                        this.delay = 10;
                        obj.init_smoke(celeste, 0.0, 0.0);

                        let floordex = obj.check(celeste, "FallFloor", 0.0, 1.0);
                        if let Some(i) = floordex {
                            let oref = celeste.objects[i].clone();
                            let mut floorobj = oref.borrow_mut();
                            let fref = match &mut floorobj.obj_type {
                                ObjectType::FallFloor(p) => p.clone(),
                                _ => unreachable!(),
                            };
                            let mut floor = fref.borrow_mut();
                            this.hide_in = 15; // TODO: innacuracy: break_floor doesn't hide it
                                               // because then they would both have a mut ref, impossible in safe rust
                            floor.break_floor(&mut floorobj, celeste);
                        }
                        // psfx 8
                    }
                }
                None => (),
            }
        } else if this.delay > 0 {
            this.delay -= 1;
            if this.delay <= 0 {
                obj.spr = 18;
            }
        }
        if this.hide_in > 0 {
            this.hide_in -= 1;
            if this.hide_in <= 0 {
                this.hide_for = 60;
                obj.spr = 0;
            }
        }
    }
    pub fn draw(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::Spring(p) => p.clone(),
            _ => unreachable!(),
        };
        let _this = tref.borrow_mut();

        obj.draw_sprite(celeste);
    }
}
