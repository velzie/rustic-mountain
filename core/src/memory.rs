use crate::{
    structures::{FlipState, Vector},
    utils::{max, mid, min},
    Celeste,
};
use rand::{rngs::ThreadRng, thread_rng};
use serde::{Deserialize, Serialize};
use std::io;
#[derive(Serialize, Deserialize)]
pub struct Memory {
    #[serde(skip, default = "deflogger")]
    pub logger: Box<dyn Fn(&str)>,
    pub graphics: Vec<u8>,
    pub fontatlas: Vec<bool>,
    pub map: Vec<u8>,
    pub sprites: Vec<u8>,
    pub flags: Vec<u8>,
    pub buttons: Vec<bool>,

    pub pallete: Vec<ColorState>,
    pub camera: Vector,
    #[serde(skip, default = "defrng")]
    pub rng: ThreadRng,
}
fn deflogger() -> Box<dyn Fn(&str)> {
    Box::new(dlog)
}
fn defrng() -> ThreadRng {
    thread_rng()
}
fn dlog(t: &str) {
    println!("{}", t);
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorState {
    pub color: u8,
    pub transparent: bool,
}
impl Memory {
    pub fn new(map: String, sprites: String, flags: String, fontatlas: String) -> Memory {
        let random = thread_rng();
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
            fontatlas: fontatlas.chars().map(|c| c == '0').collect(),
            map: hex::decode(map).unwrap(),
            sprites: sprites
                .chars()
                .map(|c| u8::from_str_radix(&format!("{}", c), 16).unwrap())
                .collect(),
            flags: hex::decode(flags).unwrap(),
            pallete: pal,
            rng: thread_rng(),
            camera: Vector { x: 0.0, y: 0.0 },
        }
    }
    pub fn spr(&mut self, sprite: u8, x: i32, y: i32, flip: Option<FlipState>) {
        let flip = flip.unwrap_or(FlipState { x: false, y: false });
        for i in 0..8 {
            for j in 0..8 {
                let mut ci = i;
                let mut cj = j;
                if flip.x {
                    ci = 7 - i;
                }
                if flip.y {
                    cj = 7 - j;
                }
                let color = self.pallete[self.sprites[(((sprite as usize % 16) * 8)
                    + (((sprite as usize / 16) * 8 * 128) + ci + (cj * 128)))]
                    as usize]
                    .clone();

                if !color.transparent {
                    self.pset(color.color, x + i as i32, y + j as i32);
                }
            }
        }
    }
    pub fn map(&mut self, celx: u8, cely: u8, sx: i32, sy: i32, celw: u8, celh: u8, mask: u8) {
        for ioffset in 0..celw {
            for joffset in 0..celh {
                let sprnum = self.mget(celx + ioffset, cely + joffset);
                let flag = self.fget_all(sprnum);
                if (flag & mask) == mask {
                    // if sprnum == 11 || sprnum == 12 {
                    //     dbg!((celx + ioffset, cely + joffset));
                    // }
                    self.spr(
                        sprnum,
                        (sx + ioffset as i32) * 8,
                        (sy + joffset as i32) * 8,
                        None,
                    );
                }
            }
        }
    }
    pub fn circfill(&mut self, xc: u8, yc: u8, r: i8, c: u8) {
        let mut x: i8 = 0;
        let mut y: i8 = r as i8;
        let mut d: i8 = 3 - 2 * r;
        self.draw_circ(xc as i32, yc as i32, x as i32, y as i32, c);
        while y >= x as i8 {
            x += 1;
            if d > 0 {
                y -= 1;
                d = d + 4 * (x - y) + 10;
            } else {
                d = d + 4 * x + 6;
            }
            self.draw_circ(xc as i32, yc as i32, x as i32, y as i32, c);
        }
    }
    fn draw_circ(&mut self, xc: i32, yc: i32, x: i32, y: i32, c: u8) {
        self.rectfill(
            (xc - x).into(),
            (yc + y).into(),
            (xc - x).into(),
            (yc - y).into(),
            c,
        );
        self.rectfill(
            (xc + y).into(),
            (yc + x).into(),
            (xc - y).into(),
            (yc + x).into(),
            c,
        );
        self.rectfill(
            (xc + x).into(),
            (yc - y).into(),
            (xc + x).into(),
            (yc + y).into(),
            c,
        );
        self.rectfill(
            (xc - y).into(),
            (yc - x).into(),
            (xc + y).into(),
            (yc - x).into(),
            c,
        );
        self.rectfill(
            (xc + x).into(),
            (yc - y).into(),
            (xc + x).into(),
            (yc + y).into(),
            c,
        );
        self.rectfill(
            (xc - y).into(),
            (yc - x).into(),
            (xc + y).into(),
            (yc - x).into(),
            c,
        );
    }
    pub fn rrectfill(&mut self, x: i32, y: i32, x1: i32, y1: i32, col: u8) {
        let mut i = x;
        loop {
            let mut j = y;
            loop {
                self.pset(col, i, j);
                j += (y1 - y).signum();
                if j == y1 {
                    break;
                }
            }
            i += (x1 - x).signum();
            if i == x1 {
                break;
            }
        }
    }
    pub fn camera(&mut self, x: f32, y: f32) {
        self.camera = Vector { x, y };
    }
    pub fn rectfill(&mut self, x: i32, y: i32, x2: i32, y2: i32, c: u8) {
        if x < 128 && x2 > 0 && y < 128 && y2 > 0 {
            self.rrectfill(0.max(x), 0.max(y), x2.min(127), y2.min(127), c);
        }
    }

    pub fn pal(&mut self, index: usize, color: u8) {
        self.pallete[index].color = color;
    }
    pub fn palt(&mut self, index: usize, transparent: bool) {
        self.pallete[index].transparent = transparent;
    }
    pub fn print(&mut self, text: &str, x: i32, y: i32, col: u8) {
        for (c, chr) in text.char_indices() {
            let char_index = chr as usize;
            for i in 0..3 {
                for j in 0..5 {
                    if self.fontatlas[(char_index / 16 * (128 * 8))
                        + (char_index % 16 * 8)
                        + i as usize
                        + (j as usize * 128)]
                    {
                        self.pset(col, x + i + (c as i32 * 4), y + j);
                    }
                }
            }
        }
    }
    pub fn pset(&mut self, col: u8, mut x: i32, mut y: i32) {
        x += self.camera.x as i32;
        y += self.camera.y as i32;
        if x < 0 || y < 0 || x >= 128 || y >= 128 {
            return;
        }
        self.graphics[x as usize + y as usize * 128] = col;
    }

    pub fn mget(&self, x: u8, y: u8) -> u8 {
        let ind = x as usize + y as usize * 128;
        if ind < 4096 {
            self.map[ind]
        } else {
            // after the 4096th octet, we start reading from the shared memory section at the bottom of sprites
            // meaning we convert from 2 16s to 1 256
            let start = (ind - 4096) * 2 + 4096 * 2;

            self.sprites[start + 1] * 16 + self.sprites[start] // need to swap the nibbles
        }
    }
    pub fn mset(&mut self, x: u8, y: u8, tile: u8) {
        let ind = x as usize + y as usize * 128;
        if ind < 4096 {
            self.map[ind] = tile;
        } else {
            let start = (ind - 4096) * 2 + 4096 * 2;

            self.sprites[start] = tile % 16;
            self.sprites[start + 1] = tile / 16;
        }
    }
    pub fn fget(&self, sprnum: u8, idx: u8) -> bool {
        (self.flags[sprnum as usize] & 2 ^ idx) != 0
    }
    pub fn fget_all(&self, sprnum: u8) -> u8 {
        self.flags[sprnum as usize]
    }
}
