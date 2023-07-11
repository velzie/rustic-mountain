use std::{cell::RefCell, rc::Rc};

use rand::Rng;

use crate::{structures::*, Celeste};
use serde::{Deserialize, Serialize};

use super::orb::Orb;

#[derive(Serialize, Deserialize)]
pub struct BigChest {
    state: u8,
    timer: f32,
    particles: Vec<ChestParticle>,
}

#[derive(Serialize, Deserialize)]
struct ChestParticle {
    x: f32,
    y: f32,
    h: f32,
    spd: f32,
}
impl BigChest {
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
                h: 8.0,
            },
            flip: FlipState { x: false, y: false },
            collidable: true,
            solids: false,
            obj_type: ObjectType::BigChest(Rc::new(RefCell::new(Self {
                state: 0,
                timer: 0.0,
                particles: vec![],
            }))),
            draw: ObjFunc(Self::draw),
            update: ObjFunc(Self::update),
            name: "BigChest",
        }
    }
    pub fn update(_obj: &mut Object, _celeste: &mut Celeste) {}
    pub fn draw(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::BigChest(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();
        if this.state == 0 {
            let hit = obj.check(celeste, "Player", 0.0, 8.0);
            match hit {
                Some(i) => {
                    let jref = celeste.objects[i].clone();
                    let mut playerobj = jref.borrow_mut();
                    if playerobj.is_solid(0.0, 1.0, celeste) {
                        // music -1 500 7
                        // sfx 37
                        celeste.pause_player = true;
                        playerobj.spd = Vector { x: 0.0, y: 0.0 };
                        this.state = 1;
                        obj.init_smoke(celeste, 0.0, 0.0);
                        obj.init_smoke(celeste, 8.0, 0.0);
                        this.timer = 60.0;
                    }
                }
                _ => (),
            }
            celeste
                .mem
                .spr(96, obj.pos.x as i32, obj.pos.y as i32, None);
            celeste
                .mem
                .spr(97, obj.pos.x as i32 + 8, obj.pos.y as i32, None);
        } else if this.state == 1 {
            this.timer -= 1.0;
            celeste.shake = 5;
            celeste.flash_bg = true;
            if this.timer <= 45.0 && this.particles.len() < 50 {
                this.particles.push(ChestParticle {
                    x: celeste.mem.rng.gen_range(1.0..15.0),
                    y: 0.0,
                    h: celeste.mem.rng.gen_range(32.0..64.0),
                    spd: celeste.mem.rng.gen_range(8.0..16.0),
                });
            }
            if this.timer < 0.0 {
                this.state = 2;
                this.particles.clear();
                celeste.flash_bg = false;
                celeste.new_bg = true;
                let obj = Rc::new(RefCell::new(Orb::init(
                    celeste,
                    obj.pos.x + 4.0,
                    obj.pos.y + 4.0,
                )));
                celeste.objects.push(obj);
                celeste.pause_player = false;
            }
            for particle in &mut this.particles {
                particle.y += particle.spd;
                celeste.mem.rectfill(
                    (obj.pos.x + particle.x) as i32,
                    (obj.pos.y + 8.0 - particle.y) as i32,
                    (obj.pos.x + particle.x) as i32,
                    (obj.pos.y + 8.0).min(obj.pos.y + 8.0 - particle.y + particle.h) as i32,
                    7,
                )
            }
        }
        celeste
            .mem
            .spr(112, obj.pos.x as i32, obj.pos.y as i32 + 8, None);
        celeste
            .mem
            .spr(113, obj.pos.x as i32 + 8, obj.pos.y as i32 + 8, None);
    }
}
