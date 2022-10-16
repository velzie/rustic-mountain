use std::cell::RefCell;
use std::rc::Rc;

use rand::Rng;

use crate::utils::mid;
use crate::{memory::Memory, structures::*, utils::*, Celeste};


pub struct Balloon {
    offset: f32,
    timer: f32,
    start: f32,
}
impl Balloon {
    pub fn init(celeste: &mut Celeste, x: f32, y: f32) -> Object {
        Object {
            pos: Vector { x, y },
            spd: Vector { x: 0.0, y: 0.0 },
            rem: Vector { x: 0.0, y: 0.0 },
            spr: 22,
            hitbox: Rectangle {
                x: -1.0,
                y: -1.0,
                w: 10.0,
                h: 10.0,
            },
            flip: FlipState { x: false, y: false },
            collidable: true,
            solids: false,
            obj_type: ObjectType::Balloon(Rc::new(RefCell::new(Self {
                offset: celeste.mem.rng.gen_range(0.0..1.0),
                timer: 0.0,
                start: y,
            }))),
            draw: Self::draw,
            update: Self::update,
            name: "Balloon",
        }
    }
    fn update(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::Balloon(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();
        if obj.spr == 22 {
            this.offset += 0.01;
            obj.pos.y = this.start + this.offset.sin() * 2.0;
            let hit = obj.check(celeste, "Player", 0.0, 0.0);
            match hit {
                Some(i) => {
                    let mut playerobj = celeste.objects[i].borrow_mut();
                    let pref = match &mut playerobj.obj_type {
                        ObjectType::Player(p) => p.clone(),
                        _ => unreachable!(),
                    };
                    let mut player = pref.borrow_mut();
                    //psfx 6
                    // obj.init_smoke(x, y)
                    player.djump = celeste.max_djump;
                    obj.spr = 0;
                    this.timer = 60.0;
                }
                None => (),
            }
        } else if this.timer > 0.0 {
            this.timer -= 1.0;
        } else {
            // psfx 7
            // obj.init_smoke(None)
            obj.spr = 22;
        }
    }
    fn draw(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::Balloon(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();
        if obj.spr == 22 {
            celeste.mem.spr(
                (13.0 + (this.offset * 8.0) % 3.0) as u8,
                obj.pos.x as i32,
                (obj.pos.y + 6.0) as i32,
                None,
            );
            obj.draw_sprite(celeste);
        }
    }
}