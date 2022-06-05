#![feature(core_intrinsics)]

pub mod memory;
pub mod objects;
pub mod structures;
pub mod utils;
use std::{cell::RefCell, rc::Rc};

use memory::Memory;
use objects::Player;
use structures::*;
pub struct Celeste<'a> {
    pub mem: Memory<'a>,
    pub objects: Vec<Box<dyn Object>>,
    pub got_fruit: u8,
    pub max_djump: u8,
    pub deaths: u8,
    pub frames: u64,
}

impl<'a> Celeste<'a> {
    pub fn new(map: String, sprites: String, flags: String) -> Celeste<'a> {
        let mut cel = Celeste {
            mem: Memory::new(map, sprites, flags),
            objects: vec![],
            got_fruit: 0,
            max_djump: 1,
            deaths: 0,
            frames: 0,
        };

        let player = Box::new(Player::init(&mut cel));
        cel.objects.push(player);
        cel
    }
    // pub fn init(&'a mut self) {

    //     let s = Some(self);
    //     self.mem.celeste = s;
    // }

    pub fn next_tick(&mut self) {
        self.frames += 1;

        for i in 0..self.objects.len() {
            let v = self.objects.get_mut(i).unwrap();
            // i.update(&self);
            v.update(&mut self.mem);
        }
        // let graph = &mut rself.borrow_mut().mem.graphics;
        // for i in 0..128 * 128 {
        //     // graphics[(i % 15) as u8] = i as ;
        // }
    }
    pub fn draw(&mut self) {
        for i in 0..128 * 128 {
            self.mem.graphics[i] = (i % 15) as u8;
        }
        for i in 0..self.objects.len() {
            let v = self.objects.get_mut(i).unwrap();
            // i.update(&self);
            v.draw(&mut self.mem);
        }
        // really draw shouldn't change state but it could
        // let rself = RefCell::new(self);
        // for object in &mut rself.borrow_mut().objects {
        //     object.draw(*rself.borrow_mut());
        // }
    }

    fn check(
        &mut self,
        obj: &Box<dyn Object>,
        name: &'static str,
        x: f32,
        y: f32,
    ) -> Option<&mut Box<dyn Object>> {
        for other in &mut self.objects {
            if other.name() == name && *other.collidable() {
                if other.right() >= obj.left() + x
                    && other.bottom() >= obj.top()
                    && other.left() <= obj.right() + x
                    && other.top() <= obj.bottom() + y
                {
                    return Some(other);
                }
            }
        }
        None
    }
}
