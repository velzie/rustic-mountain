pub mod memory;
pub mod objects;
pub mod structures;
pub mod utils;
use std::{cell::RefCell, rc::Rc};

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
}

impl Celeste {
    pub fn new(map: String, sprites: String, flags: String) -> Celeste {
        let mut cel = Celeste {
            mem: Memory::new(map, sprites, flags),
            objects: vec![],
            got_fruit: 0,
            max_djump: 1,
            deaths: 0,
            frames: 0,
        };

        let player = Rc::new(RefCell::new(Player::init(&mut cel)));
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
            let v = self.objects[i].clone();
            v.borrow_mut().do_move();
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
        self.mem.map(0, 0, 0, 0, 16, 16, 4);
        for i in 0..self.objects.len() {
            let v = self.objects[i].clone();
            v.borrow_mut().draw(self);
        }
    }
    fn check(
        &mut self,
        object: &Rc<RefCell<dyn Object>>,
        name: &'static str,
        x: f32,
        y: f32,
    ) -> Option<&Rc<RefCell<dyn Object>>> {
        let obj = object.borrow_mut();
        for o in &mut self.objects {
            let other = o.borrow_mut();
            if other.name() == name && *other.collidable() {
                if other.right() >= obj.left() + x
                    && other.bottom() >= obj.top()
                    && other.left() <= obj.right() + x
                    && other.top() <= obj.bottom() + y
                {
                    return Some(o);
                }
            }
        }
        None
    }
}
