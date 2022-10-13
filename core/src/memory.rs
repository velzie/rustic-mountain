use std::io;

use crate::Celeste;

pub struct Memory {
    pub logger: Box<dyn Fn(&str)>,
    pub graphics: Vec<u8>,
    pub fontatlas: Vec<bool>,
    pub map: Vec<u8>,
    pub sprites: Vec<u8>,
    pub flags: Vec<u8>,
    pub buttons: Vec<bool>,
    pub pallete: Vec<ColorState>,
}
#[derive(Debug, Clone)]
pub struct ColorState {
    pub color: u8,
    pub transparent: bool,
}
impl Memory {
    pub fn new(map: String, sprites: String, flags: String, fontatlas: String) -> Memory {
        let mut graphics = vec![];
        for i in 0..128 * 128 {
            graphics.push((i % 15) as u8);
        }
        let mut pal = vec![];
        for i in 0..16 {
            pal.push(ColorState {
                color: i,
                transparent: false,
            })
        }
        pal[0].transparent = true;
        Memory {
            logger: Box::new(|s| println!("{}", s)),
            buttons: vec![false; 6],
            graphics,
            fontatlas: sprites.chars().map(|c| c == '1').collect(),
            map: hex::decode(map).unwrap(),
            sprites: sprites
                .chars()
                .map(|c| u8::from_str_radix(&format!("{}", c), 16).unwrap())
                .collect(),
            flags: hex::decode(flags).unwrap(),
            pallete: pal,
        }
    }
    pub fn spr(&mut self, sprite: u8, x: u8, y: u8) {
        //sprite
        for i in 0..8 {
            for j in 0..8 {
                let color = self.pallete[self.sprites[(((sprite as usize % 16) * 8)
                    + (((sprite as usize / 16) * 8 * 128) + i + (j * 128)))]
                    as usize]
                    .clone();

                if !color.transparent {
                    self.gset(color.color, x + i as u8, y + j as u8);
                }
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
    pub fn rectfill(&mut self, x: u8, y: u8, x1: u8, y1: u8, col: u8) {
        for i in x..x1 + 1 {
            for j in y..y1 + 1 {
                self.gset(col, i, j);
            }
        }
    }
    pub fn pal(&mut self, index: usize, color: u8) {
        self.pallete[index].color = color;
    }
    pub fn palt(&mut self, index: usize, transparent: bool) {
        self.pallete[index].transparent = transparent;
    }
    pub fn print(&mut self, text: String, x: u8, y: u8, col: u8) {
        for (i, chr) in text.char_indices() {
            for i in 0..5 {
                for j in 0..3 {
                    // if ()
                }
            }
        }
    }
    pub fn gset(&mut self, col: u8, x: u8, y: u8) {
        if x >= 128 || y >= 128 {
            // print!("out of range");
            return;
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
