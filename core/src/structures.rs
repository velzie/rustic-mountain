use std::{cell::RefCell, rc::Rc};

use crate::{memory::Memory, Celeste};

pub struct Vector {
    pub x: f32,
    pub y: f32,
}
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

fn mutateCeleste(cel: &mut Celeste) {
    cel.deaths += 1;
}

pub trait Object {
    fn pos(&self) -> &Vector;
    fn spd(&self) -> &Vector;
    fn rem(&self) -> &Vector;
    fn spr(&self) -> &u8;
    fn hitbox(&self) -> &Rectangle;

    fn collidable(&self) -> &bool;
    fn name(&self) -> &'static str;

    fn pos_mut(&mut self) -> &mut Vector;
    fn spd_mut(&mut self) -> &mut Vector;
    fn rem_mut(&mut self) -> &mut Vector;
    fn spr_mut(&mut self) -> &mut u8;
    fn hitbox_mut(&mut self) -> &mut Rectangle;

    fn init(celeste: &mut Celeste) -> Self
    where
        Self: Sized;
    fn update(&mut self, celeste: &mut Memory);
    fn draw(&mut self, celeste: &mut Memory);

    fn left(&self) -> f32 {
        self.pos().x + self.hitbox().x
    }
    fn right(&self) -> f32 {
        self.left() + self.hitbox().w - 1f32
    }
    fn top(&self) -> f32 {
        self.pos().y + self.hitbox().y
    }
    fn bottom(&self) -> f32 {
        self.top() + self.hitbox().h - 1f32
    }
    fn init_smoke(&self, x: f32, y: f32) {
        // do later
    }
    fn do_move(&mut self, x: f32, y: f32) {}
    // fn is_solid(x:f32,y:f32) -> bool{
    //     return y > 0 and
    // }
    // fn is_flag(x:f32,y:f32,) ->
    // fn check(name:&'static str,x:f32,y:f32,celeste: &mut Celeste) -> bool{
    //     for obj in
    // }
}
