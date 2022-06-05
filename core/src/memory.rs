use crate::Celeste;

pub struct Memory<'a> {
    pub celeste: Option<&'a mut Celeste<'a>>,
    pub graphics: Vec<u8>,
    pub map: Vec<u8>,
    pub sprites: Vec<u8>,
    pub flags: Vec<u8>,
    pub buttons: Vec<bool>,
}

impl<'a> Memory<'a> {
    pub fn new(map: String, sprites: String, flags: String) -> Memory<'a> {
        let mut graphics = vec![];
        for i in 0..128 * 128 {
            graphics.push((i % 15) as u8);
        }
        Memory {
            celeste: None,
            buttons: vec![false; 5],
            graphics: graphics,
            map: hex::decode(map).unwrap(),
            sprites: hex::decode(sprites).unwrap(),
            flags: hex::decode(flags).unwrap(),
        }
    }
    pub fn spr(&mut self, sprite: u8, x: u8, y: u8) {
        //sprite
        for i in 0..8 {
            for j in 0..8 {
                self.gset(
                    self.sprites
                        [(((sprite % 16) * 8) + (((sprite / 16) * 128) + i + j * 128)) as usize],
                    x + i,
                    y + j,
                );
            }
        }
    }
    pub fn gset(&mut self, col: u8, x: u8, y: u8) {
        self.graphics[(x + y * 128) as usize] = col;
    }

    pub fn mget(&self, x: u8, y: u8) -> u8 {
        self.map[(x + y * 128) as usize]
    }
    pub fn mset(&mut self, x: u8, y: u8, tile: u8) {
        self.map[(x + y * 128) as usize] = tile
    }
    pub fn fget(&self, x: u8, y: u8) -> u8 {
        self.flags[(x + y * 128) as usize]
    }
}
