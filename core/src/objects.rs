use std::cell::RefCell;

use crate::{memory::Memory, structures::*, Celeste};
pub struct Player {
    pub pos: Vector,
    pub spd: Vector,
    pub rem: Vector,
    pub spr: u8,
    pub hitbox: Rectangle,
    pub collidable: bool,
    pub name: &'static str,

    pub grace: f32,
    pub jbuffer: f32,
    pub djump: u8,
    pub dash_time: f32,
    pub dash_effect_time: f32,
    pub dash_target_effect: f32,
    pub dash_target_x: f32,
    pub dash_target_y: f32,
    pub dash_accel_x: f32,
    pub dash_accel_y: f32,
    pub spr_off: u8,
    pub solids: bool,
}

impl Object for Player {
    fn init(celeste: &mut Celeste) -> Player {
        Player {
            pos: Vector { x: 0f32, y: 0f32 },
            rem: Vector { x: 0f32, y: 0f32 },
            spd: Vector { x: 0f32, y: 0f32 },
            spr: 1,
            collidable: true,
            grace: 0f32,
            jbuffer: 0f32,
            dash_accel_x: 0f32,
            dash_time: 0f32,
            dash_accel_y: 0f32,
            dash_effect_time: 0f32,
            dash_target_effect: 0f32,
            dash_target_x: 0f32,
            dash_target_y: 0f32,
            spr_off: 0,

            name: "Player",
            djump: celeste.max_djump,
            hitbox: Rectangle {
                x: 1f32,
                y: 3f32,
                w: 6f32,
                h: 5f32,
            },
            solids: true,
        }
    }
    fn update(&mut self, mem: &mut Memory) {
        // if true || self.{//spikes at

        // }
        if mem.buttons[0] {
            self.pos.x -= 1f32;
        }
        if mem.buttons[1] {
            self.pos.x += 1f32;
        }
        if mem.buttons[2] {
            self.pos.y -= 1f32;
        }
        if mem.buttons[3] {
            self.pos.y += 1f32;
        }
    }
    fn draw(&mut self, mem: &mut Memory) {
        mem.spr(self.spr, self.pos.x as u8, self.pos.y as u8 + 200)
    }

    // "fields"
    // yeah, really stupid but its a workaround for traits not having fields
    // reaching java levels of boilerplate here, remember i need to do this for every object lmao
    // plus the borrow checkers gonna complain as soon as i want to do literally anything
    // unless i refactor to use refcells but :\
    fn pos(&self) -> &Vector {
        &self.pos
    }
    fn spd(&self) -> &Vector {
        &self.spd
    }
    fn rem(&self) -> &Vector {
        &self.rem
    }
    fn spr(&self) -> &u8 {
        &self.spr
    }
    fn hitbox(&self) -> &Rectangle {
        &self.hitbox
    }

    fn pos_mut(&mut self) -> &mut Vector {
        &mut self.pos
    }
    fn spd_mut(&mut self) -> &mut Vector {
        &mut self.spd
    }
    fn rem_mut(&mut self) -> &mut Vector {
        &mut self.rem
    }
    fn spr_mut(&mut self) -> &mut u8 {
        &mut self.spr
    }
    fn hitbox_mut(&mut self) -> &mut Rectangle {
        &mut self.hitbox
    }
    fn collidable(&self) -> &bool {
        &self.collidable
    }
    fn name(&self) -> &'static str {
        self.name
    }
}
