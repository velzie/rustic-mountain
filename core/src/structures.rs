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
    fn update(&mut self, celeste: &mut Celeste);
    fn draw(&mut self, celeste: &mut Celeste);

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
    fn do_move(&mut self) {
        self.rem_mut().x += self.spd().x;
        let amt = (self.rem().x + 0.5).floor();
        self.rem_mut().x -= amt;
        self.move_x(amt, 0f32);

        self.rem_mut().y += self.spd().y;
        let amt = (self.rem().y + 0.5).floor();
        self.rem_mut().y -= amt;
        self.move_y(amt);

        //   obj.move_y=function(amount)
        //     if obj.solids then
        //       local step = sign(amount)
        //       for i=0,abs(amount) do
        //        if not obj.is_solid(0,step) then
        //           obj.y += step
        //         else
        //           obj.spd.y = 0
        //           obj.rem.y = 0
        //           break
        //         end
        //       end
        //     else
        //       obj.y += amount
        //     end
        //   end
    }
    fn move_x(&mut self, amt: f32, start: f32) {
        //   obj.move_x=function(amount,start)
        //     if obj.solids then
        //       local step = sign(amount)
        //       for i=start,abs(amount) do
        //         if not obj.is_solid(step,0) then
        //           obj.x += step
        //         else
        //           obj.spd.x = 0
        //           obj.rem.x = 0
        //           break
        //         end
        //       end
        //     else
        //       obj.x += amount
        //     end
        //   end

        self.pos_mut().x += amt;
    }
    fn move_y(&mut self, amt: f32) {
        self.pos_mut().y += amt;
    }
    // fn is_solid(x:f32,y:f32) -> bool{
    //     return y > 0 and
    // }
    // fn is_flag(x:f32,y:f32,) ->
    // fn check(name:&'static str,x:f32,y:f32,celeste: &mut Celeste) -> bool{
    //     for obj in
    // }
}
