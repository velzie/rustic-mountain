use std::cell::RefCell;

use crate::{memory::Memory, structures::*, utils::*, Celeste};
pub struct Player {
    pub pos: Vector,
    pub spd: Vector,
    pub rem: Vector,
    pub spr: u8,
    pub flip: Vector,
    pub hitbox: Rectangle,
    pub collidable: bool,
    pub p_jump: bool,
    pub p_dash: bool,
    pub name: &'static str,

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
    pub spr_off: u8,
    pub solids: bool,
}

impl Object for Player {
    fn init(celeste: &mut Celeste, x: f32, y: f32) -> Player {
        Player {
            pos: Vector { x, y },
            rem: Vector { x: 0f32, y: 0f32 },
            spd: Vector { x: 0f32, y: 0f32 },
            flip: Vector { x: 1f32, y: 0f32 },
            spr: 1,
            collidable: true,
            grace: 0,
            jbuffer: 0,
            dash_accel_x: 0f32,
            dash_time: 0,
            dash_accel_y: 0f32,
            dash_effect_time: 0,
            dash_target_effect: 0f32,
            dash_target_x: 0f32,
            dash_target_y: 0f32,
            spr_off: 0,
            p_jump: false,
            p_dash: false,

            name: "Player",
            djump: celeste.max_djump,
            hitbox: Rectangle {
                x: 1f32,
                y: 3f32,
                w: 6f32,
                h: 4f32,
            },
            solids: true,
        }
    }
    fn update(&mut self, celeste: &mut Celeste) {
        let h_input = if celeste.mem.buttons[0] {
            -1
        } else if celeste.mem.buttons[1] {
            1
        } else {
            0
        };

        if false || self.pos.y > 128f32 { // spike kill
             // kill_player(self)
        }

        let on_ground = self.is_solid(0f32, 2f32, celeste);

        let jump = celeste.mem.buttons[4] && !self.p_jump;
        self.p_jump = celeste.mem.buttons[4];
        let dash = celeste.mem.buttons[5] && !self.p_dash;
        self.p_dash = celeste.mem.buttons[5];

        if jump {
            self.jbuffer = 4
        } else if self.jbuffer > 0 {
            self.jbuffer -= 1;
        }
        // (celeste.mem.logger)(&format!("g: {}", self.grace));
        if on_ground {
            self.grace = 6;
            if self.djump < celeste.max_djump {
                self.djump = celeste.max_djump;
            }
        } else if self.grace > 0 {
            self.grace -= 1
        }

        self.dash_effect_time -= 1;
        if self.dash_time > 0 {
            // init smoke
            self.dash_time -= 1;
            self.spd = Vector {
                x: appr(self.spd.x, self.dash_target_x, self.dash_accel_x),
                y: appr(self.spd.y, self.dash_target_y, self.dash_accel_y), // do something here idk
            }
        }

        let maxrun = 1f32;
        let decel = 0.15f32;
        // replace with on ice
        let accel = if false {
            0.05f32
        } else {
            if on_ground {
                0.6
            } else {
                0.4
            }
        };

        self.spd.x = if self.spd.x.abs() <= maxrun {
            appr(self.spd.x, h_input as f32 * maxrun, accel)
        } else {
            appr(self.spd.x, sign(self.spd.x) * maxrun, decel)
        };
        if self.spd.x.abs() != 0f32 {
            self.flip.x = if self.spd.x < 0f32 { 1f32 } else { 0f32 };
        }
        // log(self.spd.x);

        // if h_input == 0 &&

        //    -- facing direction
        //    if this.spd.x~=0 then
        //     this.flip.x=this.spd.x<0
        //    end
        // y movement

        let mut maxfall = 2f32;

        if h_input != 0 && self.is_solid(h_input as f32, 0f32, celeste) {
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
            self.spd.y = appr(
                self.spd.y,
                maxfall,
                if self.spd.y.abs() > 0.15f32 {
                    0.21
                } else {
                    0.105
                },
            )
        }

        if self.jbuffer > 0 {
            if self.grace > 1 {
                self.jbuffer = 0;
                self.grace = 0;
                self.spd.y = -2f32;
            } else {
                let wall_dir = if self.is_solid(-3f32, 0f32, celeste) {
                    -1f32
                } else if self.is_solid(3f32, 0f32, celeste) {
                    1f32
                } else {
                    0f32
                };
                self.jbuffer = 0;
                if wall_dir != 0f32 {
                    self.spd = Vector {
                        x: wall_dir * (-1f32 - maxrun),
                        y: -2f32,
                    };
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

        let d_full = 5f32;
        let d_half = 3.5355339059;
        if self.djump > 0 && dash {
            // (celeste.mem.logger)("??");
            // init smoke
            self.djump -= 1;
            self.dash_time = 4;
            celeste.has_dashed = true;
            self.dash_effect_time = 10;

            let v_input = if celeste.mem.buttons[2] {
                -1
            } else if celeste.mem.buttons[3] {
                1
            } else {
                0
            };

            self.spd = Vector {
                x: if h_input != 0 {
                    h_input as f32 * (if v_input != 0 { d_half } else { d_full })
                } else {
                    if v_input != 0 {
                        0f32
                    } else {
                        // if self.flip.x == -1f32 {
                        //     -1f32
                        // } else {
                        //     1f32
                        // }
                        0f32
                    }
                },
                y: if v_input != 0 {
                    v_input as f32 * if h_input != 0 { d_half } else { d_full }
                } else {
                    0f32
                },
            };

            self.dash_target_x = 2f32 * sign(self.spd.x);
            self.dash_target_y =
                (if self.spd.y >= 0f32 { 2f32 } else { 1.5f32 }) * sign(self.spd.y);
            self.dash_accel_x = if self.spd.y == 0f32 {
                1.5f32
            } else {
                1.06066017177f32
            };
            self.dash_accel_y = if self.spd.x == 0f32 {
                1.5f32
            } else {
                1.06066017177f32
            };
            // this.dash_accel_x=this.spd.y==0 and 1.5 or 1.06066017177 -- 1.5 * sqrt()
            //     this.dash_accel_y=this.spd.x==0 and 1.5 or 1.06066017177
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
        // self.
    }
    fn draw(&mut self, celeste: &mut Celeste) {
        // self.spr += 1;
        celeste
            .mem
            .spr(self.spr, self.pos.x as u8, self.pos.y as u8);
        // celeste
        //     .mem
        //     .spr(self.spr, self.pos.x as u8, self.pos.y as u8 + 200)
    }

    // "fields"
    // yeah, really stupid but its a workaround for traits not having fields
    // reaching java levels of boilerplate here, remember i need to do this for every object lmao
    // plus the borrow checkers gonna complain as soon as i want to do literally anything
    // unless i refactor to use refcells but :\
    fn pos(&self) -> &Vector {
        &self.pos
    }
    fn spd(&self) -> &Vector {
        &self.spd
    }
    fn rem(&self) -> &Vector {
        &self.rem
    }
    fn spr(&self) -> &u8 {
        &self.spr
    }
    fn hitbox(&self) -> &Rectangle {
        &self.hitbox
    }

    fn pos_mut(&mut self) -> &mut Vector {
        &mut self.pos
    }
    fn spd_mut(&mut self) -> &mut Vector {
        &mut self.spd
    }
    fn rem_mut(&mut self) -> &mut Vector {
        &mut self.rem
    }
    fn spr_mut(&mut self) -> &mut u8 {
        &mut self.spr
    }
    fn hitbox_mut(&mut self) -> &mut Rectangle {
        &mut self.hitbox
    }
    fn collidable(&self) -> &bool {
        &self.collidable
    }
    fn name(&self) -> &'static str {
        self.name
    }
}
