use std::cell::RefCell;
use std::rc::Rc;




use crate::{structures::*, utils::*, Celeste};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Key {}
impl Key {
    pub fn init(_celeste: &mut Celeste, x: f32, y: f32) -> Object {
        Object {
            pos: Vector { x, y },
            spd: Vector { x: 0.0, y: 0.0 },
            rem: Vector { x: 0.0, y: 0.0 },
            spr: 22,
            hitbox: Rectangle {
                x: 0.0,
                y: 0.0,
                w: 8.0,
                h: 8.0,
            },
            flip: FlipState { x: false, y: false },
            collidable: true,
            solids: false,
            obj_type: ObjectType::Key(Rc::new(RefCell::new(Self {}))),
            draw: Self::draw,
            update: Self::update,
            name: "Key",
        }
    }
    fn update(obj: &mut Object, celeste: &mut Celeste) {
        obj.spr = (9.5 + sin(celeste.frames as f32 / 30.0)).floor() as u8;
        if celeste.frames == 18 {
            obj.flip.x = !obj.flip.x;
        }
        if obj.check(celeste, "Player", 0.0, 0.0).is_some() {
            // sfx shit
            celeste.has_key = true;
            obj.destroy_self(celeste);
        }
    }
    fn draw(obj: &mut Object, celeste: &mut Celeste) {
        obj.draw_sprite(celeste);
    }
}
