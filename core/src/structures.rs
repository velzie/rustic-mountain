use std::{cell::RefCell, rc::Rc};

// #[macro_use]

use crate::{
    memory::Memory,
    objects::{
        balloon::Balloon, bigchest::BigChest, chest::Chest, fakewall::FakeWall,
        fallfloor::FallFloor, flag::Flag, flyfruit::FlyFruit, fruit::Fruit, key::Key,
        lifeup::LifeUp, platform::Platform, player::Player, playerspawn::PlayerSpawn, smoke::Smoke,
        spring::Spring,
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
        if self.solids {
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
            self.pos.y += amt;
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
            || self.check(celeste, "FallFloor", x, y).is_some()
            || self.check(celeste, "FakeWall", x, y).is_some();
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
    }

    /// and then they turned themself into a strawberry. funniest shit i've ever seen
    pub fn init_fruit(&mut self, celeste: &mut Celeste, ox: f32, oy: f32) {
        // sfx shit
        let fruit = Fruit::init(celeste, self.pos.x + ox, self.pos.y + oy);
        celeste.objects.push(Rc::new(RefCell::new(fruit)));
        self.destroy_self(celeste);
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
    FlyFruit(Rc<RefCell<FlyFruit>>),
    LifeUp(Rc<RefCell<LifeUp>>),
    FakeWall(Rc<RefCell<FakeWall>>),
    Key(Rc<RefCell<Key>>),
    Chest(Rc<RefCell<Chest>>),
}
