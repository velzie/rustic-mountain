use std::io;

use crate::Celeste;

pub struct Memory {
    pub logger: Box<dyn Fn(&str)>,
    pub graphics: Vec<u8>,
    pub map: Vec<u8>,
    pub sprites: Vec<u8>,
    pub flags: Vec<u8>,
    pub buttons: Vec<bool>,
}

impl Memory {
    pub fn new(map: String, sprites: String, flags: String) -> Memory {
        let mut graphics = vec![];
        for i in 0..128 * 128 {
            graphics.push((i % 15) as u8);
        }
        Memory {
            logger: Box::new(|s| println!("{}", s)),
            buttons: vec![false; 6],
            graphics: graphics,
            map: hex::decode(map).unwrap(),
            sprites: sprites
                .chars()
                .map(|c| u8::from_str_radix(&format!("{}", c), 16).unwrap())
                .collect(),
            flags: hex::decode(flags).unwrap(),
        }
    }
    pub fn spr(&mut self, sprite: u8, x: u8, y: u8) {
        //sprite
        for i in 0..8 {
            for j in 0..8 {
                self.gset(
                    self.sprites[(((sprite as usize % 16) * 8)
                        + (((sprite as usize / 16) * 8 * 128) + i + (j * 128)))],
                    x + i as u8,
                    y + j as u8,
                );
            }
        }
    }
    pub fn map(&mut self, celx: u8, cely: u8, sx: u8, sy: u8, celw: u8, celh: u8, mask: u8) {
        for ioffset in 0..celw {
            for joffset in 0..celh {
                let sprnum = self.mget(celx + ioffset, cely + joffset);
                let flag = self.fget_all(sprnum);
                if (flag & mask) == mask {
                    self.spr(sprnum, (sx + ioffset) * 8, (sy + joffset) * 8);
                }
            }
        }
    }
    pub fn gset(&mut self, col: u8, x: u8, y: u8) {
        if x > 128 || y > 128 {
            panic!("out of range")
        }
        self.graphics[x as usize + y as usize * 128] = col;
    }

    pub fn mget(&self, x: u8, y: u8) -> u8 {
        self.map[x as usize + y as usize * 128]
    }
    pub fn mset(&mut self, x: u8, y: u8, tile: u8) {
        self.map[x as usize + y as usize * 128] = tile
    }
    pub fn fget(&self, sprnum: u8, idx: u8) -> bool {
        (self.flags[sprnum as usize] & 2 ^ idx) != 0
    }
    pub fn fget_all(&self, sprnum: u8) -> u8 {
        self.flags[sprnum as usize]
    }
}
