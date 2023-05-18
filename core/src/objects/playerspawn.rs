use std::{cell::RefCell, rc::Rc};

use crate::{structures::*, Celeste};

use super::player::Player;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlayerSpawn {
    target: f32,
    state: u8,
    delay: i8,
    djump: u8,
}
impl PlayerSpawn {
    pub fn init(celeste: &mut Celeste, x: f32, y: f32) -> Object {
        Object {
            pos: Vector { x, y: 128.0 },
            spd: Vector { x: 0.0, y: -4.0 },
            rem: Vector { x: 0.0, y: 0.0 },
            spr: 3,
            hitbox: Rectangle {
                x: 0.0,
                y: 0.0,
                w: 0.0,
                h: 0.0,
            },
            flip: FlipState { x: false, y: false },
            collidable: false,
            solids: false,
            obj_type: ObjectType::PlayerSpawn(Rc::new(RefCell::new(Self {
                delay: 0,
                state: 0,
                djump: celeste.max_djump,
                target: y,
            }))),
            draw: Self::draw,
            update: Self::update,
            name: "PlayerSpawn",
        }
        //sfx 4
    }
    fn update(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::PlayerSpawn(p) => p.clone(),
            _ => unreachable!(),
        };

        let mut this = tref.borrow_mut();
        if this.state == 0 {
            //&& obj.pos.y < this.target + 16.0
            this.state = 1;
            this.delay = 3;
        } else if this.state == 1 {
            obj.spd.y += 0.5;
            if obj.spd.y > 0.0 {
                if this.delay > 0 {
                    obj.spd.y = 0.0;
                    this.delay -= 1;
                } else if obj.pos.y > this.target {
                    obj.pos.y = this.target;
                    obj.spd = Vector { x: 0.0, y: 0.0 };
                    this.state = 2;
                    this.delay = 5;
                    celeste.shake = 5;
                    // init smoke
                    // sfx 5
                }
            }
        } else if this.state == 2 {
            this.delay -= 1;
            obj.spr = 6;
            if this.delay < 0 {
                let player = Player::init(celeste, obj.pos.x, this.target);
                celeste.objects.push(Rc::new(RefCell::new(player)));
                obj.destroy_self(celeste);
            }
        }
    }
    fn draw(obj: &mut Object, celeste: &mut Celeste) {
        obj.draw_sprite(celeste);
    }
}
