#[macro_use]
extern crate lazy_static;

pub mod memory;
pub mod objects;
pub mod structures;
#[macro_use]
pub mod utils;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use memory::Memory;
use objects::Player;
use structures::*;

pub struct Celeste {
    pub mem: Memory,
    pub objects: Vec<Rc<RefCell<dyn Object>>>,
    pub got_fruit: u8,
    pub max_djump: u8,
    pub deaths: u8,
    pub frames: u64,
    pub room: Vector,
    pub level: u8,
    pub has_dashed: bool,
    pub has_key: bool,
}

impl Celeste {
    pub fn new(map: String, sprites: String, flags: String) -> Celeste {
        // let v: Box<dyn Fn(&mut Celeste) -> Box<dyn Object>> =
        //     ;

        let mut cel = Celeste {
            room: Vector { x: 0f32, y: 0f32 },
            mem: Memory::new(map, sprites, flags),
            objects: vec![],
            got_fruit: 0,
            max_djump: 1,
            deaths: 0,
            frames: 0,
            level: 0,
            has_key: false,
            has_dashed: false,
        };
        cel.next_room();
        cel
    }
    // pub fn load_level();
    pub fn next_tick(&mut self) {
        self.frames += 1;

        for i in 0..self.objects.len() {
            let v = self.objects[i].clone();
            v.borrow_mut().do_move(self);
            v.borrow_mut().update(self);
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
        //clearing screen
        // TODO:
        // title screen
        // reset palette
        // clouds

        self.mem.map(
            self.room.x as u8 * 16,
            self.room.y as u8 * 16,
            0,
            0,
            16,
            16,
            4,
        );

        self.mem.map(
            self.room.x as u8 * 16,
            self.room.y as u8 * 16,
            0,
            0,
            16,
            16,
            2,
        );
        for i in 0..self.objects.len() {
            let v = self.objects[i].clone();
            v.borrow_mut().draw(self);
        }

        // do particles here
    }
    fn next_room(&mut self) {
        // do sound at some point
        self.load_room(self.level % 8, self.level / 8);
    }
    fn load_room(&mut self, x: u8, y: u8) {
        self.objects.clear();

        self.room = Vector {
            x: x as f32,
            y: y as f32,
        };

        for i in 0..15 {
            for j in 0..15 {
                let tile = self.mem.mget(x * 16 + i, y * 16 + j);
                (self.mem.logger)(&format!("{}", tile));
                match match tile {
                    1 => Some(Player::init(self, i as f32 * 8f32, j as f32 * 8f32)),
                    _ => None,
                } {
                    Some(o) => {
                        self.objects.push(Rc::new(RefCell::new(o)));
                    }
                    None => (),
                }

                //
            }
        }
    }
}
