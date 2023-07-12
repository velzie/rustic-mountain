use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::mid;
use crate::DeadParticle;
use crate::{structures::*, utils::*, Celeste};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]

pub struct Player {
    pub grace: u8,
    pub jbuffer: u8,
    pub djump: u8,
    pub dash_time: i32,
    pub dash_effect_time: u8,
    pub dash_target_effect: f32,
    pub dash_target_x: f32,
    pub dash_target_y: f32,
    pub dash_accel_x: f32,
    pub dash_accel_y: f32,
    pub spr_off: f32,
    pub was_on_ground: bool,
    pub hair: Vec<Vector>,
    pub p_jump: bool,
    pub p_dash: bool,
}
impl Player {
    pub fn init(celeste: &mut Celeste, x: f32, y: f32) -> Object {
        Object {
            draw: ObjFunc(Self::draw),
            update: ObjFunc(Self::update),
            pos: Vector { x, y },
            spd: Vector { x: 0.0, y: 0.0 },
            rem: Vector { x: 0.0, y: 0.0 },
            flip: FlipState { x: false, y: false },
            spr: 1,
            hitbox: Rectangle {
                x: 1.0,
                y: 3.0,
                w: 6.0,
                h: 5.0,
            },
            collidable: true,
            solids: true,
            obj_type: ObjectType::Player(Rc::new(RefCell::new(Self {
                grace: 0,
                jbuffer: 0,
                dash_accel_x: 0f32,
                dash_time: 0,
                dash_accel_y: 0f32,
                dash_effect_time: 0,
                dash_target_effect: 0f32,
                dash_target_x: 0f32,
                dash_target_y: 0f32,
                spr_off: 0.0,
                p_jump: false,
                p_dash: false,
                hair: vec![Vector { x, y }; 4],
                djump: celeste.max_djump,
                was_on_ground: false,
            }))),
            name: "Player",
        }
    }
    pub fn update(obj: &mut Object, celeste: &mut Celeste) {
        if celeste.pause_player {
            return;
        }
        let tref = match &mut obj.obj_type {
            ObjectType::Player(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();
        let h_input = if celeste.mem.buttons[0] {
            -1
        } else if celeste.mem.buttons[1] {
            1
        } else {
            0
        };

        if celeste.spikes_at(
            obj.left(),
            obj.top(),
            obj.right(),
            obj.bottom(),
            obj.spd.x,
            obj.spd.y,
        ) || obj.pos.y > 128.0
        {
            // spike kill
            this.kill(obj, celeste);
        }

        let on_ground = obj.is_solid(0.0, 1.0, celeste);

        if on_ground && !this.was_on_ground {
            obj.init_smoke(celeste, 0.0, 4.0);
        }

        let jump = celeste.mem.buttons[4] && !this.p_jump;
        this.p_jump = celeste.mem.buttons[4];
        let dash = celeste.mem.buttons[5] && !this.p_dash;
        this.p_dash = celeste.mem.buttons[5];

        if jump {
            this.jbuffer = 4
        } else if this.jbuffer > 0 {
            this.jbuffer -= 1;
        }
        // (celeste.mem.logger)(&format!("g: {}", self.grace));
        if on_ground {
            this.grace = 6;
            if this.djump < celeste.max_djump {
                // sfx 54
                this.djump = celeste.max_djump;
            }
        } else if this.grace > 0 {
            this.grace -= 1;
        }
        if this.dash_effect_time > 0 {
            this.dash_effect_time -= 1;
        }
        // self.dash_effect_time -= 1;
        if this.dash_time > 0 {
            obj.init_smoke(celeste, 0.0, 0.0);
            this.dash_time -= 1;
            obj.spd = Vector {
                x: appr(obj.spd.x, this.dash_target_x, this.dash_accel_x),
                y: appr(obj.spd.y, this.dash_target_y, this.dash_accel_y), // do something here idk
            }
        } else {
            let maxrun = 1.0;
            let decel = 0.15;
            // replace with on ice
            let accel = if false {
                0.05
            } else {
                if on_ground {
                    0.6
                } else {
                    0.4
                }
            };

            obj.spd.x = if obj.spd.x.abs() <= maxrun {
                appr(obj.spd.x, h_input as f32 * maxrun, accel)
            } else {
                appr(obj.spd.x, sign(obj.spd.x) * maxrun, decel)
            };
            if obj.spd.x.abs() != 0f32 {
                obj.flip.x = obj.spd.x < 0f32;
            }

            let mut maxfall = 2.0;

            if h_input != 0 && obj.is_solid((h_input * 2) as f32, 0f32, celeste) {
                maxfall = 0.4;
            }
            if !on_ground {
                obj.spd.y = appr(
                    obj.spd.y,
                    maxfall,
                    if obj.spd.y.abs() > 0.15 { 0.21 } else { 0.105 },
                )
            }

            if this.jbuffer > 0 {
                if this.grace > 0 {
                    this.jbuffer = 0;
                    this.grace = 0;
                    obj.spd.y = -2f32;
                    obj.init_smoke(celeste, 0.0, 4.0)
                } else {
                    let wall_dir = if obj.is_solid(-3f32, 0f32, celeste) {
                        -1f32
                    } else if obj.is_solid(3f32, 0f32, celeste) {
                        1f32
                    } else {
                        0f32
                    };
                    if wall_dir != 0f32 {
                        // psfx 2
                        this.jbuffer = 0;
                        obj.spd = Vector {
                            x: wall_dir * (-1f32 - maxrun),
                            y: -2f32,
                        };
                        if !obj.is_ice(wall_dir * 3.0, 0.0, celeste) {
                            obj.init_smoke(celeste, wall_dir * 6.0, 0.0);
                        }
                    }
                }
            }

            let d_full = 5.0;
            let d_half = 3.5355339059;
            if this.djump > 0 && dash {
                obj.init_smoke(celeste, 0.0, 0.0);
                this.djump -= 1;
                this.dash_time = 4;
                celeste.has_dashed = true;
                this.dash_effect_time = 10;

                let v_input = if celeste.mem.buttons[2] {
                    -1
                } else if celeste.mem.buttons[3] {
                    1
                } else {
                    0
                };

                obj.spd = Vector {
                    x: if h_input != 0 {
                        h_input as f32 * (if v_input != 0 { d_half } else { d_full })
                    } else {
                        if v_input != 0 {
                            0f32
                        } else {
                            if obj.flip.x {
                                -1f32
                            } else {
                                1f32
                            }
                        }
                    },
                    y: if v_input != 0 {
                        v_input as f32 * if h_input != 0 { d_half } else { d_full }
                    } else {
                        0f32
                    },
                };

                celeste.freeze = 2;
                celeste.shake = 6;

                this.dash_target_x = 2.0 * sign(obj.spd.x);
                this.dash_target_y = (if obj.spd.y >= 0.0 { 2.0 } else { 1.5 }) * sign(obj.spd.y);
                this.dash_accel_x = if obj.spd.y == 0.0 { 1.5 } else { 1.06066017177 };
                this.dash_accel_y = if obj.spd.x == 0.0 { 1.5 } else { 1.06066017177 };
            }
        }

        this.spr_off += 0.25;
        obj.spr = if !on_ground {
            if obj.is_solid(h_input as f32 * 2.0, 0.0, celeste) {
                5
            } else {
                3
            }
        } else {
            if celeste.mem.buttons[3] {
                6
            } else if celeste.mem.buttons[2] {
                7
            } else {
                if obj.spd.x != 0.0 && h_input != 0 {
                    (1.0 + this.spr_off % 4.0) as u8
                } else {
                    1
                }
            }
        };
        if obj.pos.y < -4.0 {
            celeste.next_room();
        }
        this.was_on_ground = on_ground;
    }
    pub fn draw(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::Player(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();
        let djump = this.djump.clone();

        draw_player(obj, celeste, &mut this.hair, djump);
    }
    pub fn kill(&mut self, obj: &mut Object, celeste: &mut Celeste) {
        obj.destroy_self(celeste);
        celeste.dead_particles.clear();
        let mut i: f32 = 0.0;
        loop {
            celeste.dead_particles.push(DeadParticle {
                x: obj.pos.x + 4.0,
                y: obj.pos.y + 4.0,
                t: 2.0,
                dx: sin(i) * 3.0f32,
                dy: cos(i) * 3.0f32,
            });

            if i >= 0.875 {
                break;
            }
            i += 0.125;
        }
        celeste.delay_restart = 15;
    }
}
pub fn draw_player(obj: &mut Object, celeste: &mut Celeste, hair: &mut Vec<Vector>, djump: u8) {
    let clamped = mid(obj.pos.x, -1.0, 121.0);
    if obj.pos.x != clamped {
        obj.pos.x = clamped;
        obj.spd.x = 0.0;
    }

    let haircol = if djump == 1 {
        8
    } else if djump == 0 {
        12
    } else if celeste.frames % 6 < 3 {
        7
    } else {
        11
    };
    celeste.mem.pal(8, haircol);

    let mut first = Vector {
        x: obj.pos.x + if obj.flip.x { 6.0 } else { 2.0 },
        y: obj.pos.y + if celeste.mem.buttons[3] { 4.0 } else { 3.0 },
    };
    let mut last = &mut first;
    for (i, h) in hair.iter_mut().enumerate() {
        h.x += (last.x - h.x) / 1.5;
        h.y += (last.y + 0.5 - h.y) / 1.5;
        celeste
            .mem
            .circfill(h.x as u8, h.y as u8, (3 - i).min(2).max(1) as i8, haircol);
        last = h;
    }
    obj.draw_sprite(celeste);
    celeste.mem.pal(8, 8);
}
