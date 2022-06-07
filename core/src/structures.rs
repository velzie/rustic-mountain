use std::{cell::RefCell, rc::Rc};

// #[macro_use]

use crate::{memory::Memory, utils::*, Celeste};

// use crate::utils::log;

macro_rules! log {
    ($x:expr,$y:expr) => {
        ($x.mem.logger)(&format!("{}", $y))
    };
}

// use crate::utils::
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

    fn init(celeste: &mut Celeste, x: f32, y: f32) -> Self
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
    fn do_move(&mut self, celeste: &mut Celeste) {
        self.rem_mut().x += self.spd().x;
        let amt = (self.rem().x + 0.5).floor();
        self.rem_mut().x -= amt;
        if true {
            let step = sign(amt);
            for i in 0..amt.abs() as i32 + 1 {
                self.pos_mut().x += step;
                if self.is_solid(step, 0f32, celeste) {
                    self.pos_mut().x -= step;
                    self.spd_mut().x = 0f32;
                    self.rem_mut().x = 0f32;
                    break;
                }
            }
        } else {
            self.pos_mut().x += amt;
        }

        self.rem_mut().y += self.spd().y;
        let amt = (self.rem().y + 0.5).floor();
        self.rem_mut().y -= amt;
        if true {
            let step = sign(amt);
            for i in 0..amt.abs() as i32 + 1 {
                self.pos_mut().y += step;
                if self.is_solid(step, 0f32, celeste) {
                    self.pos_mut().y -= step;
                    self.spd_mut().y = 0f32;
                    self.rem_mut().y = 0f32;
                    break;
                }
            }
        } else {
            self.pos_mut().x += amt;
        }
    }
    fn check(
        &mut self,
        celeste: &mut Celeste,
        name: &'static str,
        x: f32,
        y: f32,
    ) -> Option<usize> {
        let obj = self;
        for i in 0..celeste.objects.len() {
            match celeste.objects[i].try_borrow() {
                Ok(other) => {
                    if other.name() == name && *other.collidable() {
                        if other.right() >= obj.left() + x
                            && other.bottom() >= obj.top()
                            && other.left() <= obj.right() + x
                            && other.top() <= obj.bottom() + y
                        {
                            return Some(i);
                        }
                    }
                }
                Err(_) => {}
            };
        }
        None
    }
    // fn move_x(&mut self, amt: f32, start: f32, celeste: &mut Celeste) {
    //     //solids?
    //     if true {
    //         let step = sign(amt);
    //         for i in 0..amt.abs() as i32 + 1 {
    //             if self.is_solid(step, 0f32, celeste) {
    //                 self.spd_mut().x = 0f32;
    //                 self.rem_mut().x = 0f32;
    //                 break;
    //             } else {
    //                 self.pos_mut().x += step;
    //             }
    //         }
    //     } else {
    //         self.pos_mut().x += amt;
    //     }
    // }
    // fn move_y(&mut self, amt: f32, celeste: &mut Celeste) {
    //     if true {
    //         let step = sign(amt);
    //         for i in 0..amt.abs() as i32 + 1 {
    //             self.pos_mut().y += step;
    //             if self.is_solid(0f32, 0f32, celeste) {
    //                 self.pos_mut().y -= step;
    //                 self.spd_mut().y = 0f32;
    //                 self.rem_mut().y = 0f32;
    //                 break;
    //             }
    //         }
    //     } else {
    //         self.pos_mut().y += amt;
    //     }
    //     //   obj.move_y=function(amount)
    //     //     if obj.solids then
    //     //       local step = sign(amount)
    //     //       for i=0,abs(amount) do
    //     //        if not obj.is_solid(0,step) then
    //     //           obj.y += step
    //     //         else
    //     //           obj.spd.y = 0
    //     //           obj.rem.y = 0
    //     //           break
    //     //         end
    //     //       end
    //     //     else
    //     //       obj.y += amount
    //     //     end
    //     //   end
    // }
    fn is_solid(&mut self, x: f32, y: f32, celeste: &mut Celeste) -> bool {
        // log!(celeste, "d");
        return self.is_flag(x, y, 8, celeste);
        // return (y > 0f32
        //     && self.check(celeste, "platform", x, 0f32).is_none()
        //     && self.check(celeste, "platform", x, y).is_some())
        //     || self.is_flag(x, y, 0, celeste)
        //     || self.check(celeste, "fall_floor", x, y).is_some()
        //     || self.check(celeste, "fake_wall", x, y).is_some();
    }
    fn is_flag(&mut self, x: f32, y: f32, flag: u8, celeste: &mut Celeste) -> bool {
        // log!(celeste, max(0f32, (self.left() + x) / 8f32) as i32);
        // log!(celeste, min(15f32, (self.right() + x) / 8f32) as i32);

        for i in (max(0f32, (self.left() + x) / 8f32) as i32)
            ..(min(15f32, (self.right() + x) / 8f32) as i32) + 1
        {
            for j in max(0f32, (self.top() + y) / 8f32) as i32
                ..min(15f32, self.bottom() + y / 8f32) as i32 + 1
            {
                let fg = celeste
                    .mem
                    .fget_all(celeste.mem.mget((i as f32) as u8, (j as f32) as u8)); //celeste.room.y * 16f32/celeste.room.x * 16f32 +

                // log!(celest)
                // log!(celeste, celeste.mem.mget(i as u8, j as u8));
                // log!(celeste, i);
                log!(celeste, format!("({} {})", i, j));
                if fg == 3 {
                    return true;
                }
            }
        }
        //         for i=max(0,(obj.left()+ox)\8),min(15,(obj.right()+ox)/8) do
        //    for j=max(0,(obj.top()+oy)\8),min(15,(obj.bottom()+oy)/8) do
        //     if fget(tile_at(i,j),flag) then
        //      return true
        //     end
        //    end
        false
    }
    // fn check(name:&'static str,x:f32,y:f32,celeste: &mut Celeste) -> bool{
    //     for obj in
    // }
}
