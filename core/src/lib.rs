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
    pub fn new(map: String, sprites: String, flags: String, fontatlas: String) -> Celeste {
        // let v: Box<dyn Fn(&mut Celeste) -> Box<dyn Object>> =
        //     ;

        let mut cel = Celeste {
            room: Vector { x: 0f32, y: 0f32 },
            mem: Memory::new(map, sprites, flags, fontatlas),
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

            let obj = v.borrow_mut();
            let spd = Vector {
                x: obj.spd().x,
                y: obj.spd().y,
            };
            drop(obj);
            v.borrow_mut().do_move(self, spd.x, spd.y, 0f32);
            v.borrow_mut().update(self);
        }
        // let graph = &mut rself.borrow_mut().mem.graphics;
        // for i in 0..128 * 128 {
        //     // graphics[(i % 15) as u8] = i as ;
        // }
    }
    pub fn draw(&mut self) {
        for i in 0..128 * 128 {
            self.mem.graphics[i] = 0; // (i % 15) as u8;
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
        self.level += 1;
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
    pub fn tile_at(&self, x: u8, y: u8) -> u8 {
        return self
            .mem
            .mget(self.room.x as u8 * 16 + x, self.room.y as u8 * 16 + y);
    }
    pub fn spikes_at(&self, x1: f32, y1: f32, x2: f32, y2: f32, xspd: f32, yspd: f32) -> bool {
        let mut i = utils::max(0f32, x1 / 8f32);
        loop {
            let mut j = utils::max(0f32, y1 / 8f32);
            if i > utils::min(15f32, x2 / 8f32) {
                break;
            }
            loop {
                if j > utils::min(15f32, y2 / 8f32) {
                    break;
                }
                if match self.tile_at(i as u8, j as u8) {
                    17 => yspd >= 0f32 && y2 % 8f32 >= 6f32,
                    27 => yspd <= 0f32 && y1 % 8f32 <= 2f32,
                    43 => xspd <= 0f32 && x1 % 8f32 <= 2f32,
                    59 => xspd >= 0f32 && x2 % 8f32 >= 6f32,
                    _ => false,
                } {
                    return true;
                }
                j += 1f32;
            }
            i += 1f32;
        }
        return false;
    }
    //     function spikes_at(x1,y1,x2,y2,xspd,yspd)
    //  for i=max(0,x1\8),min(15,x2/8) do
    //   for j=max(0,y1\8),min(15,y2/8) do
    //    if ({[17]=yspd>=0 and y2%8>=6,
    //     [27]=yspd<=0 and y1%8<=2,
    //     [43]=xspd<=0 and x1%8<=2,
    //     [59]=xspd>=0 and x2%8>=6})[tile_at(i,j)] then
    //     return true
    //    end
    //   end
    //  end
    // end
}
