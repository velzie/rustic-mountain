use std::{cell::RefCell, rc::Rc};

// #[macro_use]

use crate::{
    memory::Memory,
    objects::{
        balloon::Balloon, bigchest::BigChest, fakewall::FakeWall, fallfloor::FallFloor, flag::Flag,
        fruit::Fruit, lifeup::LifeUp, platform::Platform, player::Player, playerspawn::PlayerSpawn,
        smoke::Smoke, spring::Spring,
    },
    utils::*,
    Celeste,
};

// use crate::utils::log;

macro_rules! log {
    ($x:expr,$y:expr) => {
        ($x.mem.logger)(&format!("{}", $y))
    };
}
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}
#[derive(Serialize, Deserialize)]

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct FlipState {
    pub x: bool,
    pub y: bool,
}
#[derive(Serialize)]

pub struct Object {
    pub pos: Vector,
    pub spd: Vector,
    pub rem: Vector,
    pub spr: u8,
    pub hitbox: Rectangle,
    pub flip: FlipState,

    pub collidable: bool,
    pub solids: bool,

    pub obj_type: ObjectType,
    #[serde(skip)]
    pub draw: fn(&mut Object, &mut Celeste),
    #[serde(skip)]
    pub update: fn(&mut Object, &mut Celeste),
    pub name: &'static str,
}
impl Object {
    pub fn draw(&mut self, celeste: &mut Celeste) {
        (self.draw)(self, celeste);
    }
    pub fn update(&mut self, celeste: &mut Celeste) {
        (self.update)(self, celeste);
    }
    pub fn left(&self) -> f32 {
        self.pos.x + self.hitbox.x
    }
    pub fn right(&self) -> f32 {
        self.left() + self.hitbox.w - 1f32
    }
    pub fn top(&self) -> f32 {
        self.pos.y + self.hitbox.y
    }
    pub fn bottom(&self) -> f32 {
        self.top() + self.hitbox.h - 1f32
    }

    pub fn init_smoke(&self, celeste: &mut Celeste, x: f32, y: f32) {
        let smoke = Smoke::init(celeste, self.pos.x + x, self.pos.y + y);
        celeste.objects.push(Rc::new(RefCell::new(smoke)));
    }

    pub fn draw_sprite(&self, celeste: &mut Celeste) {
        celeste.mem.spr(
            self.spr,
            self.pos.x as i32,
            self.pos.y as i32,
            Some(self.flip.clone()),
        )
    }

    pub fn do_move(&mut self, celeste: &mut Celeste, ox: f32, oy: f32, start: f32) {
        self.rem.x += ox;
        let amt = (self.rem.x + 0.5).floor();
        self.rem.x -= amt;
        if self.solids {
            let step = sign(amt);
            let mut i = start;
            loop {
                if !self.is_solid(step, 0f32, celeste) {
                    self.pos.x += step;
                } else {
                    self.spd.x = 0f32;
                    self.rem.x = 0f32;
                    break;
                }
                if i >= amt.abs() {
                    break;
                }
                i += 1f32;
            }
        } else {
            self.pos.x += amt;
        }

        self.rem.y += oy;
        let amt = (self.rem.y + 0.5).floor();
        self.rem.y -= amt;
        if true {
            let step = sign(amt);
            let mut i = 0f32; //start
            loop {
                if !self.is_solid(0f32, step, celeste) {
                    self.pos.y += step;
                } else {
                    self.spd.y = 0f32;
                    self.rem.y = 0f32;
                    break;
                }
                if i >= amt.abs() {
                    break;
                }
                i += 1f32;
            }
        } else {
            self.pos.x += amt;
        }
    }
    pub fn check(
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
                    if other.name == name && other.collidable {
                        if other.right() >= obj.left() + x
                            && other.bottom() >= obj.top() + y
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
    pub fn is_ice(&self, x: f32, y: f32, celeste: &mut Celeste) -> bool {
        self.is_flag(x, y, 4, celeste)
    }
    pub fn is_solid(&mut self, x: f32, y: f32, celeste: &mut Celeste) -> bool {
        return (y > 0f32
            && self.check(celeste, "Platform", x, 0f32).is_none()
            && self.check(celeste, "Platform", x, y).is_some())
            || self.is_flag(x, y, 1, celeste)
            || self.check(celeste, "FallFloor", x, y).is_some();
    }
    pub fn is_flag(&self, x: f32, y: f32, flag: u8, celeste: &mut Celeste) -> bool {
        for i in max(0f32, (self.left() + x) / 8f32) as i32
            ..(min(15f32, (self.right() + x) / 8f32)) as i32 + 1
        {
            for j in max(0f32, (self.top() + y) / 8f32) as i32
                ..min(15f32, (self.bottom() + y) / 8f32) as i32 + 1
            {
                let fg = celeste.mem.fget_all(celeste.mem.mget(
                    (celeste.room.x as u8 * 16) + i as u8,
                    (celeste.room.y as u8 * 16) + j as u8,
                ));
                if (flag & fg) == flag {
                    return true;
                }
            }
        }
        false

        // let mut i = 15f32.min((self.right() + x) / 8.0);
        // dbg!(&i);
        // loop {
        //     let mut j = 0f32.max((self.top() + y) / 8.0);
        //     loop {
        //         let fg = celeste.mem.fget_all(celeste.mem.mget(
        //             ((celeste.room.x * 16.0) + i) as u8,
        //             ((celeste.room.y * 16.0) + j) as u8,
        //         ));
        //         if (flag & fg) == flag {
        //             return true;
        //         }
        //         j += 1.0;
        //         if j >= 15f32.min((self.bottom() + y) / 8.0) {
        //             break;
        //         }
        //     }
        //     dbg!(0f32.max((self.left() + x) / 8.0));
        //     i += 1.0;
        //     if i >= 0f32.max((self.right() + x) / 8.0) {
        //         break;
        //     }
        // }
        // false
    }

    /// WARNING: Only use this function if it is being called by the object itself in an update loop
    pub fn destroy_self(&mut self, celeste: &mut Celeste) {
        celeste.objects.retain(|o| o.try_borrow().is_ok());
        // this particular bit of jank will delete any object that is currently being used in memory
    }

    pub fn destroy_other(&mut self, celeste: &mut Celeste) {
        celeste.objects.retain(|objref| match objref.try_borrow() {
            Ok(obj) => obj.name == self.name && obj.pos == self.pos,
            Err(_) => true,
        });
    }
}

#[derive(Serialize, Deserialize)]
pub enum ObjectType {
    Player(Rc<RefCell<Player>>),
    PlayerSpawn(Rc<RefCell<PlayerSpawn>>),
    Balloon(Rc<RefCell<Balloon>>),
    Spring(Rc<RefCell<Spring>>),
    FallFloor(Rc<RefCell<FallFloor>>),
    Platform(Rc<RefCell<Platform>>),
    Smoke(Rc<RefCell<Smoke>>),
    BigChest(Rc<RefCell<BigChest>>),
    Flag(Rc<RefCell<Flag>>),
    Fruit(Rc<RefCell<Fruit>>),
    LifeUp(Rc<RefCell<LifeUp>>),
    FakeWall(Rc<RefCell<FakeWall>>),
}
// pub trait Object {
//     fn pos(&self) -> &Vector;
//     fn spd(&self) -> &Vector;
//     fn rem(&self) -> &Vector;
//     fn spr(&self) -> &u8;
//     fn hitbox(&self) -> &Rectangle;
//     fn flip(&self) -> &FlipState;

//     fn collidable(&self) -> &bool;
//     fn name(&self) -> &'static str;

//     fn pos_mut(&mut self) -> &mut Vector;
//     fn spd_mut(&mut self) -> &mut Vector;
//     fn rem_mut(&mut self) -> &mut Vector;
//     fn spr_mut(&mut self) -> &mut u8;

//     fn init(celeste: &mut Celeste, x: f32, y: f32) -> Self
//     where
//         Self: Sized;
//     fn update(&mut self, celeste: &mut Celeste);
//     fn draw(&mut self, celeste: &mut Celeste);

// fn left(&self) -> f32 {
//     self.pos().x + self.hitbox().x
// }
// fn right(&self) -> f32 {
//     self.left() + self.hitbox().w - 1f32
// }
// fn top(&self) -> f32 {
//     self.pos().y + self.hitbox().y
// }
// fn bottom(&self) -> f32 {
//     self.top() + self.hitbox().h - 1f32
// }
//     fn init_smoke(&self, x: f32, y: f32) {
//         // do later
//     }
// fn draw_sprite(&self, celeste: &mut Celeste) {
//     celeste.mem.spr(
//         *self.spr(),
//         self.pos().x as i32,
//         self.pos().y as i32,
//         Some((*self.flip()).clone()),
//     )
// }
// fn do_move(&mut self, celeste: &mut Celeste, ox: f32, oy: f32, start: f32) {
//     self.rem_mut().x += ox;
//     let amt = (self.rem().x + 0.5).floor();
//     self.rem_mut().x -= amt;
//     if true {
//         let step = sign(amt);
//         let mut i = start;
//         loop {
//             self.pos_mut().x += step;
//             if self.is_solid(step, 0f32, celeste) {
//                 self.pos_mut().x -= step;
//                 self.spd_mut().x = 0f32;
//                 self.rem_mut().x = 0f32;
//                 break;
//             }
//             if i >= amt.abs() {
//                 break;
//             }
//             i += 1f32;
//         }
//     } else {
//         self.pos_mut().x += amt;
//     }

//     self.rem_mut().y += oy;
//     let amt = (self.rem().y + 0.5).floor();
//     self.rem_mut().y -= amt;
//     if true {
//         let step = sign(amt);
//         let mut i = 0f32; //start
//         loop {
//             self.pos_mut().y += step;
//             if self.is_solid(0f32, step, celeste) {
//                 self.pos_mut().y -= step;
//                 self.spd_mut().y = 0f32;
//                 self.rem_mut().y = 0f32;
//                 break;
//             }
//             if i >= amt.abs() {
//                 break;
//             }
//             i += 1f32;
//         }
//     } else {
//         self.pos_mut().x += amt;
//     }
// }
// fn check(
//     &mut self,
//     celeste: &mut Celeste,
//     name: &'static str,
//     x: f32,
//     y: f32,
// ) -> Option<usize> {
//     let obj = self;
//     for i in 0..celeste.objects.len() {
//         match celeste.objects[i].try_borrow() {
//             Ok(other) => {
//                 if other.name() == name && *other.collidable() {
//                     if other.right() >= obj.left() + x
//                         && other.bottom() >= obj.top()
//                         && other.left() <= obj.right() + x
//                         && other.top() <= obj.bottom() + y
//                     {
//                         return Some(i);
//                     }
//                 }
//             }
//             Err(_) => {}
//         };
//     }
//     None
// }
//     // fn move_x(&mut self, amt: f32, start: f32, celeste: &mut Celeste) {
//     //     //solids?
//     //     if true {
//     //         let step = sign(amt);
//     //         for i in 0..amt.abs() as i32 + 1 {
//     //             if self.is_solid(step, 0f32, celeste) {
//     //                 self.spd_mut().x = 0f32;
//     //                 self.rem_mut().x = 0f32;
//     //                 break;
//     //             } else {
//     //                 self.pos_mut().x += step;
//     //             }
//     //         }
//     //     } else {
//     //         self.pos_mut().x += amt;
//     //     }
//     // }
//     // fn move_y(&mut self, amt: f32, celeste: &mut Celeste) {
//     //     if true {
//     //         let step = sign(amt);
//     //         for i in 0..amt.abs() as i32 + 1 {
//     //             self.pos_mut().y += step;
//     //             if self.is_solid(0f32, 0f32, celeste) {
//     //                 self.pos_mut().y -= step;
//     //                 self.spd_mut().y = 0f32;
//     //                 self.rem_mut().y = 0f32;
//     //                 break;
//     //             }
//     //         }
//     //     } else {
//     //         self.pos_mut().y += amt;
//     //     }
//     //     //   obj.move_y=function(amount)
//     //     //     if obj.solids then
//     //     //       local step = sign(amount)
//     //     //       for i=0,abs(amount) do
//     //     //        if not obj.is_solid(0,step) then
//     //     //           obj.y += step
//     //     //         else
//     //     //           obj.spd.y = 0
//     //     //           obj.rem.y = 0
//     //     //           break
//     //     //         end
//     //     //       end
//     //     //     else
//     //     //       obj.y += amount
//     //     //     end
//     //     //   end
//     // }
// fn is_solid(&mut self, x: f32, y: f32, celeste: &mut Celeste) -> bool {
//     // log!(celeste, "d");
//     return self.is_flag(x, y, 1, celeste);
//     // return (y > 0f32
//     //     && self.check(celeste, "platform", x, 0f32).is_none()
//     //     && self.check(celeste, "platform", x, y).is_some())
//     //     || self.is_flag(x, y, 0, celeste)
//     //     || self.check(celeste, "fall_floor", x, y).is_some()
//     //     || self.check(celeste, "fake_wall", x, y).is_some();
// }
// fn is_flag(&mut self, x: f32, y: f32, flag: u8, celeste: &mut Celeste) -> bool {
//     // log!(celeste, max(0f32, self.left() / 8f32));
//     // log!(
//     //     celeste,
//     //     max(0f32, (self.top() + y) / 8f32) as i32
//     //         - (min(15f32, (self.bottom() + y) / 8f32)) as i32
//     // );
//     // log!(celeste, min(15f32, (self.bottom() + y) / 8f32) as i32);

//     // let mut i = );
//     for i in max(0f32, (self.left() + x + 1.0) / 8f32) as i32
//         ..(min(15f32, (self.right() + x - 1.0) / 8f32)) as i32 + 1
//     {
//         for j in max(0f32, (self.top() + y + 1.0) / 8f32) as i32
//             ..min(15f32, (self.bottom() + y - 1.0) / 8f32) as i32 + 1
//         {
//             let fg = celeste.mem.fget_all(celeste.mem.mget(
//                 (celeste.room.x as u8 * 16) + i as u8,
//                 (celeste.room.y as u8 * 16) + j as u8,
//             ));
//             if (flag & fg) == flag {
//                 return true;
//             }
//         }
//     }
//         // let mut i = max(0f32, (self.left() + x) / 8f32);
//         // loop {
//         //     let mut j = max(0f32, (self.top() + y) / 8f32);
//         //     loop {
//         //         let fg = celeste.mem.fget_all(celeste.mem.mget(
//         //             (celeste.room.x as u8 * 16) + i as u8,
//         //             (celeste.room.y as u8 * 16) + j as u8,
//         //         )); //celeste.room.y * 16f32/celeste.room.x * 16f32 +

//         //         // log!(celest)
//         //         // log!(celeste, celeste.mem.mget(i as u8, j as u8));
//         //         // log!(celeste, i);
//         //         // log!(celeste, format!("({} {})", i, j));
//         //         if (flag & fg) == flag {
//         //             return true;
//         //         }
//         //         j += 1.0;
//         //         if j > min(15f32, (self.bottom() + y) / 8f32) {
//         //             break;
//         //         }
//         //     }
//         //     if i > min(15f32, (self.right() + x) / 8f32) {
//         //         break;
//         //     }
//         //     i += 1.0;
//         // }
//         //         for i=max(0,(obj.left()+ox)\8),min(15,(obj.right()+ox)/8) do
//         //    for j=max(0,(obj.top()+oy)\8),min(15,(obj.bottom()+oy)/8) do
//         //     if fget(tile_at(i,j),flag) then
//         //      return true
//         //     end
//         //    end
//         false
//     }
//     // fn check(name:&'static str,x:f32,y:f32,celeste: &mut Celeste) -> bool{
//     //     for obj in
//     // }
// }
