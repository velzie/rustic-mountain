use std::cell::RefCell;
use std::rc::Rc;

use rand::Rng;

use crate::utils::mid;
use crate::{memory::Memory, structures::*, utils::*, Celeste};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Chest {
    start: f32,
    timer: i32,
}
impl Chest {
    pub fn init(celeste: &mut Celeste, x: f32, y: f32) -> Object {
        Object {
            pos: Vector { x: x - 4.0, y },
            spd: Vector { x: 0.0, y: 0.0 },
            rem: Vector { x: 0.0, y: 0.0 },
            spr: 20,
            hitbox: Rectangle {
                x: 0.0,
                y: 0.0,
                w: 8.0,
                h: 8.0,
            },
            flip: FlipState { x: false, y: false },
            collidable: true,
            solids: false,
            obj_type: ObjectType::Chest(Rc::new(RefCell::new(Self {
                start: x - 4.0,
                timer: 20,
            }))),
            draw: Self::draw,
            update: Self::update,
            name: "Chest",
        }
    }
    fn update(obj: &mut Object, celeste: &mut Celeste) {
        if celeste.has_key {
            let tref = match &mut obj.obj_type {
                ObjectType::Chest(p) => p.clone(),
                _ => unreachable!(),
            };
            let mut this = tref.borrow_mut();
            this.timer -= 1;
            obj.pos.x = this.start - 1.0 + celeste.mem.rng.gen_range(0.0..3.0);
            if this.timer <= 0 {
                obj.init_fruit(celeste, 0.0, -4.0);
            }
        }
    }
    fn draw(obj: &mut Object, celeste: &mut Celeste) {
        obj.draw_sprite(celeste);
    }
}
