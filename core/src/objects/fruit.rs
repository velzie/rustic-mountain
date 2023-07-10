use std::cell::RefCell;
use std::rc::Rc;

use crate::{structures::*, utils::*, Celeste};

use super::lifeup::LifeUp;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Fruit {
    off: f32,
    start: f32,
}
impl Fruit {
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
            collidable: true,
            solids: false,
            obj_type: ObjectType::Fruit(Rc::new(RefCell::new(Self { start: y, off: 0.0 }))),
            draw: ObjFunc(Self::draw),
            update: ObjFunc(Self::update),
            name: "Fruit",
        }
    }
    pub fn update(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::Fruit(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();
        this.off += 0.025;
        obj.pos.y = this.start + sin(this.off) * 2.5;

        check_fruit(obj, celeste);
    }
    pub fn draw(obj: &mut Object, celeste: &mut Celeste) {
        obj.draw_sprite(celeste);
    }
}

pub fn check_fruit(obj: &mut Object, celeste: &mut Celeste) {
    match obj.check(celeste, "Player", 0.0, 0.0) {
        Some(i) => {
            let jref = celeste.objects[i].clone();
            let mut playerobj = jref.borrow_mut();
            let pref = match &mut playerobj.obj_type {
                ObjectType::Player(p) => p.clone(),
                _ => unreachable!(),
            };
            let mut player = pref.borrow_mut();
            player.djump = celeste.max_djump;
            // sfx_timer = 20
            //sfx 13
            while celeste.got_fruit.len() <= celeste.level as usize {
                celeste.got_fruit.push(false);
            }
            celeste.got_fruit[celeste.level as usize] = true;

            let lifeup = Rc::new(RefCell::new(LifeUp::init(celeste, obj.pos.x, obj.pos.y)));
            celeste.objects.push(lifeup);
            drop(player);
            drop(playerobj); //manual drop called so that player doesn't get deleted by the retain call
            obj.destroy_self(celeste);
        }
        None => (),
    }
}
