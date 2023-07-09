use std::{cell::RefCell, rc::Rc};



use crate::{structures::*, utils::sign, Celeste};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FakeWall {}
impl FakeWall {
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
            obj_type: ObjectType::FakeWall(Rc::new(RefCell::new(Self {}))),
            draw: Self::draw,
            update: Self::update,
            name: "FakeWall",
        }
    }
    fn update(obj: &mut Object, celeste: &mut Celeste) {
        // hitbox is mutated during the duration of update(). not sure why? it makes check() more
        // generous i guess
        obj.hitbox = Rectangle {
            x: -1.0,
            y: -1.0,
            w: 18.0,
            h: 18.0,
        };

        // let tref = match &mut obj.obj_type {
        //     ObjectType::FakeWall(p) => p.clone(),
        //     _ => unreachable!(),
        // };
        // let mut this = tref.borrow_mut();

        if let Some(i) = obj.check(celeste, "Player", 0.0, 0.0) {
            // rust is a systems programming language. it prioritizes speed, so i have to
            // explicitly specify when i want to actually access components i guess, leading to
            // this boilerplate every single time i want to check() a player
            let jref = celeste.objects[i].clone();
            let mut playerobj = jref.borrow_mut();
            let pref = match &mut playerobj.obj_type {
                ObjectType::Player(p) => p.clone(),
                _ => unreachable!(),
            };
            let mut player = pref.borrow_mut();
            if player.dash_effect_time > 0 {
                playerobj.spd = Vector {
                    x: sign(playerobj.spd.x) * -1.5,
                    y: -1.5,
                };
                player.dash_time = -1;
                for i in 0..2 {
                    for j in 0..2 {
                        obj.init_smoke(celeste, i as f32 * 8.0, j as f32 * 8.0)
                    }
                }
                // if we don't invoke drop manually, the player gets obliterated as soon as the
                // fruit spawns. yes, really
                drop(playerobj);
                drop(player);
                obj.init_fruit(celeste, 4.0, 4.0);
            }
        }
        obj.hitbox = Rectangle {
            x: 0.0,
            y: 0.0,
            w: 16.0,
            h: 16.0,
        };
    }
    fn draw(obj: &mut Object, celeste: &mut Celeste) {
        celeste
            .mem
            .spr(64, obj.pos.x as i32, obj.pos.y as i32, None);
        celeste
            .mem
            .spr(65, obj.pos.x as i32 + 8, obj.pos.y as i32, None);
        celeste
            .mem
            .spr(64 + 16, obj.pos.x as i32, obj.pos.y as i32 + 8, None);
        celeste
            .mem
            .spr(65 + 16, obj.pos.x as i32 + 8, obj.pos.y as i32 + 8, None);
    }
}
