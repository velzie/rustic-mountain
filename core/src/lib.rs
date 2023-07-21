pub mod assign_skipped_consts;
pub mod memory;
pub mod objects;
pub mod structures;
pub mod utils;
use std::{cell::RefCell, rc::Rc, vec};

use assign_skipped_consts::assign_skipped_consts;
use memory::Memory;
use objects::{
    balloon::Balloon, bigchest::BigChest, chest::Chest, fakewall::FakeWall, fallfloor::FallFloor,
    flag::Flag, flyfruit::FlyFruit, fruit::Fruit, key::Key, lifeup::LifeUp, message::Message,
    platform::Platform, player::Player, playerspawn::PlayerSpawn, roomtitle::RoomTitle,
    spring::Spring,
};
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::json;
use structures::*;

use rand::prelude::*;
use utils::{max, sin};
#[derive(Serialize, Deserialize)]

pub struct Celeste {
    /// Represents the pico-8 display buffers and memory. Go through this for any drawing
    pub mem: Memory,
    // #[serde(skip)]
    pub objects: Vec<Rc<RefCell<Object>>>,
    pub got_fruit: Vec<bool>,
    pub max_djump: u8,
    pub deaths: u64,
    pub frames: u8,
    pub room: Vector,
    pub level: u8,
    pub has_dashed: bool,
    pub has_key: bool,
    pub freeze: u8,
    pub particles: Vec<Particle>,
    pub dead_particles: Vec<DeadParticle>,
    pub delay_restart: u8,
    pub shake: u8,
    pub seconds: u8,
    pub minutes: u64,
    pub clouds: Vec<Cloud>,
    pub start_game_flash: f32,
    pub music_timer: i32,
    pub start_game: bool,
    pub flash_bg: bool,
    pub new_bg: bool,
    pub pause_player: bool,
}
impl Celeste {
    /// Returns a new celeste object
    /// # Arguments
    ///
    /// * `map` - A string containing the "mapdata" section of a pico-8 cart
    /// * `sprites` - A string containing the "sprites" section of the pico-8 cart
    /// * `flags` - A string containing the "flags" section of the pico-8 carts
    /// * `fontatlas` - A string with the bitmap data of the pico-8 font
    ///
    /// # Examples
    /// ```
    /// // see https://github.com/CoolElectronics/rustic-mountain/blob/main/standalone/src/consts.rs for example values of these constants
    /// let celeste = Celeste::new(MAPDATA,SPRITES,FLAGS,FONTATLAS);
    /// ```
    pub fn new(map: String, sprites: String, flags: String, fontatlas: String) -> Celeste {
        // let v: Box<dyn Fn(&mut Celeste) -> Box<dyn Object>> =
        //     ;

        let mut mem = Memory::new(map, sprites, flags, fontatlas);
        let mut clouds = vec![];
        for _ in 0..16 {
            clouds.push(Cloud {
                x: mem.rng.gen_range(0..128),
                y: mem.rng.gen_range(0..128),
                spd: mem.rng.gen_range(1..4),
                w: mem.rng.gen_range(32..64),
            })
        }
        let mut particles = vec![];
        for _i in 0..24 {
            let size: f32 = mem.rng.gen_range(0.0..1.25);
            particles.push(Particle {
                x: mem.rng.gen_range(0.0..128.0),
                y: mem.rng.gen_range(0.0..128.0),
                s: size.floor(),
                spd: mem.rng.gen_range(0.25..5.25),
                off: mem.rng.gen_range(0.0..1.0),
                c: mem.rng.gen_range(6..8),
            })
        }

        let mut cel = Celeste {
            room: Vector { x: 0f32, y: 0f32 },
            mem,
            objects: vec![],
            got_fruit: vec![],
            max_djump: 1,
            deaths: 0,
            frames: 0,
            seconds: 0,
            minutes: 0,
            level: 0,
            has_key: false,
            has_dashed: false,
            freeze: 0,
            clouds,
            particles,
            dead_particles: vec![],
            delay_restart: 0,
            shake: 0,
            start_game: false,
            music_timer: 0,
            start_game_flash: 0.0,
            flash_bg: false,
            pause_player: false,
            new_bg: false,
        };
        cel.title_screen();
        // cel.load_room(0, 0);
        cel
    }

    /// Advances a game tick. Does not draw the screen buffer. Analagous to calling `_update()` in
    /// the original pico-8 cart. Should be called 30 times a second for real-time gameplay
    pub fn next_tick(&mut self) {
        // summit
        self.frames += 1;

        if self.level < 30 {
            self.seconds += self.frames / 30;
            self.minutes += (self.seconds / 60) as u64;
            self.seconds %= 60;
        }
        self.frames %= 30;

        if self.freeze > 0 {
            self.freeze -= 1;
            return;
        }

        if self.shake > 0 {
            self.shake -= 1;
            self.mem.camera(0.0, 0.0);
            if self.shake != 0 {
                self.mem.camera = Vector {
                    x: self.mem.rng.gen_range(-2.0..3.0),
                    y: self.mem.rng.gen_range(-2.0..3.0),
                }
            }
        }

        if self.delay_restart > 0 {
            self.delay_restart -= 1;
            if self.delay_restart == 0 {
                self.load_room(self.room.x as u8, self.room.y as u8);
            }
        }

        let mut i = 0;
        let mut lastlen = self.objects.len();
        loop {
            if i >= self.objects.len() {
                break;
            }
            let v = self.objects[i].clone();

            let obj = v.borrow_mut();
            let spd = Vector {
                x: obj.spd.x,
                y: obj.spd.y,
            };
            drop(obj);

            v.borrow_mut().do_move(self, spd.x, spd.y, 0f32);
            v.borrow_mut().update(self);
            i += 1;
        }
        if self.is_title() {
            if self.start_game {
                self.start_game_flash -= 1.0;
                if self.start_game_flash <= -30.0 {
                    self.begin_game();
                }
            } else if self.mem.buttons[4] || self.mem.buttons[5] {
                // music -1
                self.start_game_flash = 50.0;
                self.start_game = true;
                // sfx 38
            }
        }
        // let graph = &mut rself.borrow_mut().mem.graphics;
        // for i in 0..128 * 128 {
        //     // graphics[(i % 15) as u8] = i as ;
        // }
    }
    pub fn is_title(&self) -> bool {
        self.level == 32
    }
    pub fn begin_game(&mut self) {
        self.max_djump = 1;
        self.deaths = 0;
        self.frames = 0;
        self.seconds = 0;
        self.minutes = 0;
        self.music_timer = 0;
        // music 007
        self.level = 0;
        self.load_room(0, 0);
    }
    pub fn draw(&mut self) {
        if self.freeze > 0 {
            return;
        }
        for i in 0..128 * 128 {
            self.mem.graphics[i] = 0; // (i % 15) as u8;
        }

        self.mem.pal_reset();

        if self.is_title() && self.start_game {
            for i in 1..16 {
                self.mem.pal(
                    i,
                    if self.start_game_flash <= 10.0 {
                        f32::ceil(max(self.start_game_flash, 0.0) / 5.0) as u8
                    } else {
                        if self.frames % 10 < 5 {
                            7
                        } else {
                            i as u8
                        }
                    },
                );
            }
        }

        //clearing screen
        let bg_col = if self.flash_bg {
            self.frames as u8 / 5
        } else {
            if self.new_bg {
                2
            } else {
                0
            }
        };
        self.mem.rrectfill(-1, -1, 128, 128, bg_col);

        if !self.is_title() {
            for cloud in &mut self.clouds {
                cloud.x += cloud.spd;
                self.mem.rectfill(
                    cloud.x,
                    cloud.y,
                    cloud.x + cloud.w,
                    cloud.y + 16 - (cloud.w as f32 * 0.1875) as i32,
                    if self.new_bg { 14 } else { 1 },
                );
                if cloud.x > 128 {
                    cloud.x = -cloud.w;
                    cloud.y = self.mem.rng.gen_range(0..120);
                }
            }
        }

        self.mem.map(
            self.room.x as u8 * 16,
            self.room.y as u8 * 16,
            0,
            0,
            16,
            16,
            4,
        );

        for v in self.objects.clone() {
            let mut r = v.borrow_mut();
            if let ObjectType::Platform(_) = r.obj_type {
                r.draw(self);
            }
        }
        self.mem.map(
            self.room.x as u8 * 16,
            self.room.y as u8 * 16,
            0,
            0,
            16,
            16,
            2, //2
        );
        for v in self.objects.clone() {
            // cloning is fine here, it's just a vector of pointers
            let mut r = v.borrow_mut();
            if let ObjectType::Platform(_) = r.obj_type {
            } else {
                r.draw(self);
            }
        }

        // do particles here
        for particle in &mut self.particles {
            particle.x += particle.spd;
            particle.y += sin(particle.off);

            self.mem.rectfill(
                particle.x as i32,
                particle.y as i32,
                (particle.x + particle.s) as i32,
                (particle.y + particle.s) as i32,
                particle.c,
            );
            if particle.x > 132.0 {
                particle.x = -4.0;
                particle.y = self.mem.rng.gen_range(0.0..128.0);
            }
        }
        for particle in &mut self.dead_particles {
            particle.x += particle.dx;
            particle.y += particle.dy;

            particle.t -= 0.2;

            if particle.t > 0.0 {
                self.mem.rectfill(
                    (particle.x - particle.t) as i32,
                    (particle.y - particle.t) as i32,
                    (particle.x + particle.t) as i32,
                    (particle.y + particle.t) as i32,
                    14 + ((particle.t * 5.0) % 2.0) as u8,
                );
            }
        }
        self.dead_particles.retain(|f| f.t > 0.0);

        if self.is_title() {
            self.mem.print("z+x", 58, 80, 5);
            // self.mem.print("(rust edition)", 41, 91, 5);
            self.mem.print("maddy thorson", 42, 96, 5);
            self.mem.print("noel berry", 46, 102, 5);
        }

        // todo: summit blinds
    }
    /// advances to the next room
    pub fn next_room(&mut self) {
        // do sound at some point
        self.level += 1;
        self.load_room(self.level % 8, self.level / 8);
    }
    pub fn title_screen(&mut self) {
        self.frames = 0;
        self.start_game_flash = 0.0;
        self.level = 32;
        // music
        self.load_room(7, 3);
    }
    pub fn load_room(&mut self, x: u8, y: u8) {
        self.objects.clear();

        self.room = Vector {
            x: x as f32,
            y: y as f32,
        };

        self.has_dashed = false;
        self.has_key = false;

        for i in 0..16 {
            for j in 0..16 {
                let tile = self.mem.mget(x * 16 + i, y * 16 + j);
                let x = i as f32 * 8.0;
                let y = j as f32 * 8.0;
                match match tile {
                    1 => Some(PlayerSpawn::init(self, x, y)),
                    11 | 12 => Some(Platform::init(self, x, y, tile)),
                    18 => Some(Spring::init(self, x, y)),
                    22 => Some(Balloon::init(self, x, y)),
                    23 => Some(FallFloor::init(self, x, y)),
                    26 | 64 | 28 | 8 | 20 => {
                        if self.got_fruit.len() > self.level as usize
                            && self.got_fruit[self.level as usize]
                        {
                            None
                        } else {
                            Some(match tile {
                                8 => Key::init(self, x, y),
                                20 => Chest::init(self, x, y),
                                26 => Fruit::init(self, x, y),
                                64 => FakeWall::init(self, x, y),
                                28 => FlyFruit::init(self, x, y),
                                _ => unreachable!(),
                            })
                        }
                    }

                    86 => Some(Message::init(self, x, y)),
                    96 => Some(BigChest::init(self, x, y)),
                    118 => Some(Flag::init(self, x, y)),
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
        if !self.is_title() {
            let obj = RoomTitle::init(self, 0.0, 0.0);
            self.objects.push(Rc::new(RefCell::new(obj)));
        }
    }
    pub fn tile_at(&self, x: f32, y: f32) -> u8 {
        return self.mem.mget(
            (self.room.x as f32 * 16.0 + x) as u8,
            (self.room.y as f32 * 16.0 + y) as u8,
        );
    }
    pub fn spikes_at(&self, x1: f32, y1: f32, x2: f32, y2: f32, xspd: f32, yspd: f32) -> bool {
        let mut i = 0f32.max((x1 / 8.0).floor()) as u8;
        loop {
            let mut j = 0f32.max((y1 / 8.0).floor()) as u8;
            loop {
                if match self.tile_at(i as f32, j as f32) {
                    17 => yspd >= 0.0 && y2.rem_euclid(8.0) >= 6.0,
                    27 => yspd <= 0.0 && y1.rem_euclid(8.0) <= 2.0,
                    43 => xspd <= 0.0 && x1.rem_euclid(8.0) <= 2.0,
                    59 => xspd >= 0.0 && x2.rem_euclid(8.0) >= 6.0,
                    _ => false,
                } {
                    return true;
                }
                if j >= 15f32.min((y2 / 8.0).floor()) as u8 {
                    break;
                }
                j += 1;
            }
            if i >= utils::min(15f32, (x2 / 8f32).floor()) as u8 {
                break;
            }
            i += 1;
        }
        return false;
    }

    /// returns a savestate in JSON format, as a string
    /// ```
    /// let savestate = celeste.save_state();
    /// celeste.load_state(savestate)
    /// ```
    pub fn save_state(&mut self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// loads a savestate in JSON format, as a string slice
    /// ```
    /// let savestate = celeste.save_state();
    /// celeste.load_state(savestate)
    /// ```
    pub fn load_state(&mut self, json: &str) {
        let deserialized: Self = serde_json::from_str(json).unwrap();
        for i in 0..deserialized.objects.len() {
            let o = &deserialized.objects[i];
            let mut obj = o.borrow_mut();
            assign_skipped_consts(&mut obj);
        }
        *self = deserialized;
    }
}
pub fn draw_time(celeste: &mut Celeste, x: i32, y: i32) {
    celeste.mem.rectfill(x, y, x + 33, y + 7, 0);
    let time = format!(
        "{}:{}:{}",
        two_digit_str(celeste.minutes / 60),
        two_digit_str(celeste.minutes % 60),
        two_digit_str(celeste.seconds as u64)
    );
    celeste.mem.print(&time, x + 1, y + 1, 7);
}
fn two_digit_str(n: u64) -> String {
    if n < 10 {
        format!("0{}", n)
    } else {
        format!("{}", n)
    }
}

#[derive(Serialize, Deserialize)]

pub struct Cloud {
    pub x: i32,
    pub y: i32,
    pub spd: i32,
    pub w: i32,
}
#[derive(Serialize, Deserialize)]

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub s: f32,
    pub spd: f32,
    pub off: f32,
    pub c: u8,
}

#[derive(Serialize, Deserialize)]
pub struct DeadParticle {
    pub x: f32,
    pub y: f32,
    pub t: f32,
    pub dx: f32,
    pub dy: f32,
}
