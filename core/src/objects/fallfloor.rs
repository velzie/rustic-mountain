use std::cell::RefCell;
use std::rc::Rc;

use rand::Rng;

use crate::utils::mid;
use crate::{memory::Memory, structures::*, utils::*, Celeste};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FallFloor {
    state: u8,
    delay: u8,
}
impl FallFloor {
    pub fn init(celeste: &mut Celeste, x: f32, y: f32) -> Object {
        Object {
            pos: Vector { x, y },
            spd: Vector { x: 0.0, y: 0.0 },
            rem: Vector { x: 0.0, y: 0.0 },
            spr: 23,
            hitbox: Rectangle {
                x: 0.0,
                y: 0.0,
                w: 8.0,
                h: 8.0,
            },
            flip: FlipState { x: false, y: false },
            collidable: true,
            obj_type: ObjectType::FallFloor(Rc::new(RefCell::new(Self { state: 0, delay: 0 }))),
            draw: Self::draw,
            update: Self::update,
            name: "FallFloor",
            solids: false,
        }
    }
    fn update(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::FallFloor(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();
        if this.state == 0 {
            for i in 0..3 {
                if obj
                    .check(celeste, "Player", (i - 1) as f32, -(i % 2) as f32)
                    .is_some()
                {
                    this.break_floor(obj, celeste);
                }
            }
        } else if this.state == 1 {
            this.delay -= 1;
            if this.delay <= 0 {
                this.state = 2;
                this.delay = 60;
                obj.collidable = false;
            }
        } else if this.state == 2 {
            this.delay -= 1;
            if this.delay <= 0 && obj.check(celeste, "Player", 0.0, 0.0).is_none() {
                // psfx 7
                this.state = 0;
                obj.collidable = true;
                obj.init_smoke(celeste, 0.0, 0.0);
            }
        }
    }
    fn draw(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::FallFloor(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();
        celeste.mem.spr(
            if this.state == 1 {
                26 - this.delay / 5
            } else if this.state == 0 {
                23
            } else {
                0
            },
            obj.pos.x as i32,
            obj.pos.y as i32,
            None,
        );
        // this.state==1 and
        // 26-this.delay/5 or
        // this.state==0 and 23
    }
    pub fn break_floor(&mut self, obj: &mut Object, celeste: &mut Celeste) {
        if self.state == 0 {
            //psfx 15
            self.state = 1;
            self.delay = 15;
            obj.init_smoke(celeste, 0.0, 0.0);
            let springdex = obj.check(celeste, "Spring", 0.0, -1.0);
            match springdex {
                Some(i) => {
                    let jref = celeste.objects[i].clone();
                    let mut springobj = jref.borrow_mut();
                    let pref = match &mut springobj.obj_type {
                        ObjectType::Spring(p) => p.clone(),
                        _ => unreachable!(),
                    };
                    pref.borrow_mut().hide_in = 15;
                }
                None => (),
            }
        }
    }
}
