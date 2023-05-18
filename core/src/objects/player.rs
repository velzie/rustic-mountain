use std::cell::RefCell;
use std::rc::Rc;

use rand::Rng;

use crate::utils::mid;
use crate::DeadParticle;
use crate::{memory::Memory, structures::*, utils::*, Celeste};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]

pub struct Player {
    pub grace: u8,
    pub jbuffer: u8,
    pub djump: u8,
    pub dash_time: u8,
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
            draw: Self::draw,
            update: Self::update,
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
    fn update(obj: &mut Object, celeste: &mut Celeste) {
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

        let on_ground = obj.is_solid(0.0, 2.0, celeste);

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
            // log(self.spd.x);

            // if h_input == 0 &&

            //    -- facing direction
            //    if this.spd.x~=0 then
            //     this.flip.x=this.spd.x<0
            //    end
            // y movement

            let mut maxfall = 2.0;

            if h_input != 0 && obj.is_solid((h_input * 2) as f32, 0f32, celeste) {
                maxfall = 0.4;
            }
            //    -- wall slide
            //    if h_input~=0 and this.is_solid(h_input,0) and not this.is_ice(h_input,0) then
            //     maxfall=0.4
            //     -- wall slide smoke
            //     if rnd()<0.2 then
            //      this.init_smoke(h_input*6)
            //     end
            //    end

            if !on_ground {
                obj.spd.y = appr(
                    obj.spd.y,
                    maxfall,
                    if obj.spd.y.abs() > 0.15 { 0.21 } else { 0.105 },
                )
            }

            if this.jbuffer > 0 {
                if this.grace > 1 {
                    this.jbuffer = 0;
                    this.grace = 0;
                    obj.spd.y = -2f32;
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

            //    -- jump
            //    if this.jbuffer>0 then
            //     if this.grace>0 then
            //      -- normal jump
            //      psfx"1"
            //      this.jbuffer=0
            //      this.grace=0
            //      this.spd.y=-2
            //      this.init_smoke(0,4)
            //     else
            //      -- wall jump
            //      local wall_dir=(this.is_solid(-3,0) and -1 or this.is_solid(3,0) and 1 or 0)
            //      if wall_dir~=0 then
            //       psfx"2"
            //       this.jbuffer=0
            //       this.spd=vector(wall_dir*(-1-maxrun),-2)
            //       if not this.is_ice(wall_dir*3,0) then
            //        -- wall jump smoke
            //        this.init_smoke(wall_dir*6)
            //       end
            //      end
            //     end
            //    end
            if obj.pos.y < -4.0 {
                celeste.next_room();
            }
            let d_full = 5.0;
            let d_half = 3.5355339059;
            if this.djump > 0 && dash {
                // (celeste.mem.logger)("??");
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

                //psfx 3
                celeste.freeze = 2;
                celeste.shake = 6;
                // celeste.s

                this.dash_target_x = 2.0 * sign(obj.spd.x);
                this.dash_target_y = (if obj.spd.y >= 0.0 { 2.0 } else { 1.5 }) * sign(obj.spd.y);
                this.dash_accel_x = if obj.spd.y == 0.0 { 1.5 } else { 1.06066017177 };
                this.dash_accel_y = if obj.spd.x == 0.0 { 1.5 } else { 1.06066017177 };
                // this.dash_accel_x=this.spd.y==0 and 1.5 or 1.06066017177 -- 1.5 * sqrt()
                //     this.dash_accel_y=this.spd.x==0 and 1.5 or 1.06066017177
            }
        }
        //    -- dash
        //    local d_full=5
        //    local d_half=3.5355339059 -- 5 * sqrt(2)

        //    if this.djump>0 and dash then
        //     this.init_smoke()
        //     this.djump-=1
        //     this.dash_time=4
        //     has_dashed=true
        //     this.dash_effect_time=10
        //     -- vertical input
        //     local v_input=btn(⬆️) and -1 or btn(⬇️) and 1 or 0
        //     -- calculate dash speeds
        //     this.spd=vector(
        //      h_input~=0 and h_input*(v_input~=0 and d_half or d_full) or (v_input~=0 and 0 or this.flip.x and -1 or 1),
        //      v_input~=0 and v_input*(h_input~=0 and d_half or d_full) or 0
        //     )
        //     -- effects
        //     psfx"3"
        //     freeze=2
        //     shake=6
        //     -- dash target speeds and accels
        //     this.dash_target_x=2*sign(this.spd.x)
        //     this.dash_target_y=(this.spd.y>=0 and 2 or 1.5)*sign(this.spd.y)
        //     this.dash_accel_x=this.spd.y==0 and 1.5 or 1.06066017177 -- 1.5 * sqrt()
        //     this.dash_accel_y=this.spd.x==0 and 1.5 or 1.06066017177
        //    elseif this.djump<=0 and dash then
        //     -- failed dash smoke
        //     psfx"9"
        //     this.init_smoke()
        //    end
        //   end

        //   -- animation
        //   this.spr_off+=0.25
        //   this.spr = not on_ground and (this.is_solid(h_input,0) and 5 or 3) or  -- wall slide or mid air
        //    btn(⬇️) and 6 or -- crouch
        //    btn(⬆️) and 7 or -- look up
        //    this.spd.x~=0 and h_input~=0 and 1+this.spr_off%4 or 1 -- walk or stand

        //   -- exit level off the top (except summit)
        //   if this.y<-4 and level_index()<31 then
        //    next_room()
        //   end
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
        this.was_on_ground = on_ground;
        //   -- was on the ground
        //   this.was_on_ground=on_ground
        //  end,

        //  draw=function(this)
        //   -- clamp in screen
        //   local clamped=mid(this.x,-1,121)
        //   if this.x~=clamped then
        //    this.x=clamped
        //    this.spd.x=0
        //   end
        //   -- draw player hair and sprite
        //   set_hair_color(this.djump)
        //   draw_hair(this)
        //   draw_obj_sprite(this)
        //   unset_hair_color()
        //  end
        // }
        // if celeste.mem.buttons[0] {
        //     self.pos.x -= 1f32;
        // }
        // if celeste.mem.buttons[1] {
        //     self.pos.x += 1f32;
        // }
        // self
    }
    fn draw(obj: &mut Object, celeste: &mut Celeste) {
        let tref = match &mut obj.obj_type {
            ObjectType::Player(p) => p.clone(),
            _ => unreachable!(),
        };
        let mut this = tref.borrow_mut();
        let clamped = mid(obj.pos.x, -1.0, 121.0);
        if obj.pos.x != clamped {
            obj.pos.x = clamped;
            obj.spd.x = 0.0;
        }

        // self.spr += 1;
        let haircol = if this.djump == 1 { 8 } else { 12 };
        celeste.mem.pal(8, haircol);

        let mut first = Vector {
            x: obj.pos.x + if obj.flip.x { 6.0 } else { 2.0 },
            y: obj.pos.y + if celeste.mem.buttons[3] { 4.0 } else { 3.0 },
        };
        // celeste.mem.circfill(50, 50, 2.into(), 3);
        let mut last = &mut first;
        for (i, h) in this.hair.iter_mut().enumerate() {
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
    pub fn kill(&mut self, obj: &mut Object, celeste: &mut Celeste) {
        obj.destroy_self(celeste);
        celeste.dead_particles.clear();
        let mut i: f32 = 0.0;
        loop {
            celeste.dead_particles.push(DeadParticle {
                x: obj.pos.x,
                y: obj.pos.y,
                t: 2.0,
                dx: i.to_degrees().sin() * 3.0f32,
                dy: i.to_degrees().cos() * 3.0f32,
            });

            if i >= 0.875 {
                break;
            }
            i += 0.125;
        }
        celeste.delay_restart = 15;
    }
}
