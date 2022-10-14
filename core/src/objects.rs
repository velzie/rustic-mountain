use std::cell::RefCell;

use rand::Rng;

use crate::utils::mid;
use crate::{memory::Memory, structures::*, utils::*, Celeste};
pub struct Player {
    pub djump:u8,
}
impl Player {
    pub fn init(celeste: &mut Celeste,x:f32,y:f32) -> Object {
        Object {
            draw: Self::draw,
            update: Self::update,
            pos: Vector { x: 1.0, y: 1.0 },
            spd: Vector { x: 0.0, y: 0.0 },
            rem: Vector { x: 0.0, y: 0.0 },
            flip: FlipState { x: false, y: false },
            spr: 1,
            hitbox: Rectangle { x: 1.0, y: 3.0, w: 6.0, h: 5.0 },
            collidable:true,
            obj_type: ObjectType::Player(Self {
                djump:0,
            }),
        }
    }
    fn update(obj: &mut Object,celeste: &mut Celeste) {
        let this = match &mut obj.obj_type{
            ObjectType::Player(p)=>p
        };
    }
    fn draw(obj: &mut Object,celeste: &mut Celeste) {
        let this = match &mut obj.obj_type{
            ObjectType::Player(p)=>p
        };

        this.djump = 2;
        obj.pos.x += 1.0;
    }

}

// pub struct Player {
//     pub pos: Vector,
//     pub spd: Vector,
//     pub rem: Vector,
//     pub spr: u8,
//     pub flip: FlipState,
//     pub hitbox: Rectangle,
//     pub collidable: bool,
//     pub p_jump: bool,
//     pub p_dash: bool,
//     pub name: &'static str,

    // pub grace: u8,
    // pub jbuffer: u8,
    // pub djump: u8,
    // pub dash_time: u8,
    // pub dash_effect_time: u8,
    // pub dash_target_effect: f32,
    // pub dash_target_x: f32,
    // pub dash_target_y: f32,
    // pub dash_accel_x: f32,
    // pub dash_accel_y: f32,
    // pub spr_off: f32,
    // pub was_on_ground: bool,
    // pub hair: Vec<Vector>,

//     pub solids: bool,
// }

// impl Object for Player {
//     fn init(celeste: &mut Celeste, x: f32, y: f32) -> Player {
//         Player {
//             pos: Vector { x, y },
//             rem: Vector { x: 0f32, y: 0f32 },
//             spd: Vector { x: 0f32, y: 0f32 },
//             spr: 1,
//             collidable: true,
//             grace: 0,
//             jbuffer: 0,
//             dash_accel_x: 0f32,
//             dash_time: 0,
//             dash_accel_y: 0f32,
//             dash_effect_time: 0,
//             dash_target_effect: 0f32,
//             dash_target_x: 0f32,
//             dash_target_y: 0f32,
//             spr_off: 0.0,
//             p_jump: false,
//             p_dash: false,
//             hair: vec![Vector { x, y }; 4],

//             name: "Player",
//             djump: celeste.max_djump,
//             hitbox: Rectangle {
//                 x: 1f32,
//                 y: 3f32,
//                 w: 6f32,
//                 h: 5f32,
//             },
//             flip: FlipState { x: false, y: false },

//             was_on_ground: false,
//             solids: true,
//         }
//     }
//     fn update(&mut self, celeste: &mut Celeste) {
//         let h_input = if celeste.mem.buttons[0] {
//             -1
//         } else if celeste.mem.buttons[1] {
//             1
//         } else {
//             0
//         };

//         if celeste.spikes_at(
//             self.left(),
//             self.top(),
//             self.right(),
//             self.bottom(),
//             self.spd.x,
//             self.spd.y,
//         ) || self.pos.y > 128.0
//         {
//             // spike kill
//             celeste.next_room();
//         }

//         let on_ground = self.is_solid(0.0, 2.0, celeste);

//         if on_ground && !self.was_on_ground {
//             // init smoke
//         }

//         let jump = celeste.mem.buttons[4] && !self.p_jump;
//         self.p_jump = celeste.mem.buttons[4];
//         let dash = celeste.mem.buttons[5] && !self.p_dash;
//         self.p_dash = celeste.mem.buttons[5];

//         if jump {
//             self.jbuffer = 4
//         } else if self.jbuffer > 0 {
//             self.jbuffer -= 1;
//         }
//         // (celeste.mem.logger)(&format!("g: {}", self.grace));
//         if on_ground {
//             self.grace = 6;
//             if self.djump < celeste.max_djump {
//                 // sfx 54
//                 self.djump = celeste.max_djump;
//             }
//         } else if self.grace > 0 {
//             self.grace -= 1;
//         }
//         if self.dash_effect_time > 0 {
//             self.dash_effect_time -= 1;
//         }
//         // self.dash_effect_time -= 1;
//         if self.dash_time > 0 {
//             // init smoke
//             self.dash_time -= 1;
//             self.spd = Vector {
//                 x: appr(self.spd.x, self.dash_target_x, self.dash_accel_x),
//                 y: appr(self.spd.y, self.dash_target_y, self.dash_accel_y), // do something here idk
//             }
//         } else {
//             let maxrun = 1.0;
//             let decel = 0.15;
//             // replace with on ice
//             let accel = if false {
//                 0.05
//             } else {
//                 if on_ground {
//                     0.6
//                 } else {
//                     0.4
//                 }
//             };

//             self.spd.x = if self.spd.x.abs() <= maxrun {
//                 appr(self.spd.x, h_input as f32 * maxrun, accel)
//             } else {
//                 appr(self.spd.x, sign(self.spd.x) * maxrun, decel)
//             };
//             if self.spd.x.abs() != 0f32 {
//                 self.flip.x = self.spd.x < 0f32;
//             }
//             // log(self.spd.x);

//             // if h_input == 0 &&

//             //    -- facing direction
//             //    if this.spd.x~=0 then
//             //     this.flip.x=this.spd.x<0
//             //    end
//             // y movement

//             let mut maxfall = 2.0;

//             if h_input != 0 && self.is_solid((h_input * 2) as f32, 0f32, celeste) {
//                 maxfall = 0.4;
//             }
//             //    -- wall slide
//             //    if h_input~=0 and this.is_solid(h_input,0) and not this.is_ice(h_input,0) then
//             //     maxfall=0.4
//             //     -- wall slide smoke
//             //     if rnd()<0.2 then
//             //      this.init_smoke(h_input*6)
//             //     end
//             //    end

//             if !on_ground {
//                 self.spd.y = appr(
//                     self.spd.y,
//                     maxfall,
//                     if self.spd.y.abs() > 0.15 { 0.21 } else { 0.105 },
//                 )
//             }

//             if self.jbuffer > 0 {
//                 if self.grace > 1 {
//                     self.jbuffer = 0;
//                     self.grace = 0;
//                     self.spd.y = -2f32;
//                 } else {
//                     let wall_dir = if self.is_solid(-3f32, 0f32, celeste) {
//                         -1f32
//                     } else if self.is_solid(3f32, 0f32, celeste) {
//                         1f32
//                     } else {
//                         0f32
//                     };
//                     self.jbuffer = 0;
//                     if wall_dir != 0f32 {
//                         self.spd = Vector {
//                             x: wall_dir * (-1f32 - maxrun),
//                             y: -2f32,
//                         };
//                     }
//                 }
//             }

//             //    -- jump
//             //    if this.jbuffer>0 then
//             //     if this.grace>0 then
//             //      -- normal jump
//             //      psfx"1"
//             //      this.jbuffer=0
//             //      this.grace=0
//             //      this.spd.y=-2
//             //      this.init_smoke(0,4)
//             //     else
//             //      -- wall jump
//             //      local wall_dir=(this.is_solid(-3,0) and -1 or this.is_solid(3,0) and 1 or 0)
//             //      if wall_dir~=0 then
//             //       psfx"2"
//             //       this.jbuffer=0
//             //       this.spd=vector(wall_dir*(-1-maxrun),-2)
//             //       if not this.is_ice(wall_dir*3,0) then
//             //        -- wall jump smoke
//             //        this.init_smoke(wall_dir*6)
//             //       end
//             //      end
//             //     end
//             //    end
//             if self.pos.y < -4.0 {
//                 celeste.next_room();
//             }
//             let d_full = 5.0;
//             let d_half = 3.5355339059;
//             if self.djump > 0 && dash {
//                 // (celeste.mem.logger)("??");
//                 // init smoke
//                 self.djump -= 1;
//                 self.dash_time = 4;
//                 celeste.has_dashed = true;
//                 self.dash_effect_time = 10;

//                 let v_input = if celeste.mem.buttons[2] {
//                     -1
//                 } else if celeste.mem.buttons[3] {
//                     1
//                 } else {
//                     0
//                 };

//                 self.spd = Vector {
//                     x: if h_input != 0 {
//                         h_input as f32 * (if v_input != 0 { d_half } else { d_full })
//                     } else {
//                         if v_input != 0 {
//                             0f32
//                         } else {
//                             // if self.flip.x == -1f32 {
//                             //     -1f32
//                             // } else {
//                             //     1f32
//                             // }
//                             0f32
//                         }
//                     },
//                     y: if v_input != 0 {
//                         v_input as f32 * if h_input != 0 { d_half } else { d_full }
//                     } else {
//                         0f32
//                     },
//                 };

//                 //psfx 3
//                 // celeste.freeze = 2;
//                 // celeste.s

//                 self.dash_target_x = 2f32 * sign(self.spd.x);
//                 self.dash_target_y =
//                     (if self.spd.y >= 0f32 { 2.0 } else { 1.5 }) * sign(self.spd.y);
//                 self.dash_accel_x = if self.spd.y == 0f32 {
//                     1.5
//                 } else {
//                     1.06066017177f32
//                 };
//                 self.dash_accel_y = if self.spd.x == 0f32 {
//                     1.5
//                 } else {
//                     1.06066017177f32
//                 };
//                 // this.dash_accel_x=this.spd.y==0 and 1.5 or 1.06066017177 -- 1.5 * sqrt()
//                 //     this.dash_accel_y=this.spd.x==0 and 1.5 or 1.06066017177
//             }
//         }
//         //    -- dash
//         //    local d_full=5
//         //    local d_half=3.5355339059 -- 5 * sqrt(2)

//         //    if this.djump>0 and dash then
//         //     this.init_smoke()
//         //     this.djump-=1
//         //     this.dash_time=4
//         //     has_dashed=true
//         //     this.dash_effect_time=10
//         //     -- vertical input
//         //     local v_input=btn(⬆️) and -1 or btn(⬇️) and 1 or 0
//         //     -- calculate dash speeds
//         //     this.spd=vector(
//         //      h_input~=0 and h_input*(v_input~=0 and d_half or d_full) or (v_input~=0 and 0 or this.flip.x and -1 or 1),
//         //      v_input~=0 and v_input*(h_input~=0 and d_half or d_full) or 0
//         //     )
//         //     -- effects
//         //     psfx"3"
//         //     freeze=2
//         //     shake=6
//         //     -- dash target speeds and accels
//         //     this.dash_target_x=2*sign(this.spd.x)
//         //     this.dash_target_y=(this.spd.y>=0 and 2 or 1.5)*sign(this.spd.y)
//         //     this.dash_accel_x=this.spd.y==0 and 1.5 or 1.06066017177 -- 1.5 * sqrt()
//         //     this.dash_accel_y=this.spd.x==0 and 1.5 or 1.06066017177
//         //    elseif this.djump<=0 and dash then
//         //     -- failed dash smoke
//         //     psfx"9"
//         //     this.init_smoke()
//         //    end
//         //   end

//         //   -- animation
//         //   this.spr_off+=0.25
//         //   this.spr = not on_ground and (this.is_solid(h_input,0) and 5 or 3) or  -- wall slide or mid air
//         //    btn(⬇️) and 6 or -- crouch
//         //    btn(⬆️) and 7 or -- look up
//         //    this.spd.x~=0 and h_input~=0 and 1+this.spr_off%4 or 1 -- walk or stand

//         //   -- exit level off the top (except summit)
//         //   if this.y<-4 and level_index()<31 then
//         //    next_room()
//         //   end
//         self.spr_off += 0.25;
//         self.spr = if !on_ground {
//             if self.is_solid(h_input as f32 * 2.0, 0.0, celeste) {
//                 5
//             } else {
//                 3
//             }
//         } else {
//             if celeste.mem.buttons[3] {
//                 6
//             } else if celeste.mem.buttons[2] {
//                 7
//             } else {
//                 if self.spd.x != 0.0 && h_input != 0 {
//                     (1.0 + self.spr_off % 4.0) as u8
//                 } else {
//                     1
//                 }
//             }
//         }
//         //   -- was on the ground
//         //   this.was_on_ground=on_ground
//         //  end,

//         //  draw=function(this)
//         //   -- clamp in screen
//         //   local clamped=mid(this.x,-1,121)
//         //   if this.x~=clamped then
//         //    this.x=clamped
//         //    this.spd.x=0
//         //   end
//         //   -- draw player hair and sprite
//         //   set_hair_color(this.djump)
//         //   draw_hair(this)
//         //   draw_obj_sprite(this)
//         //   unset_hair_color()
//         //  end
//         // }
//         // if celeste.mem.buttons[0] {
//         //     self.pos.x -= 1f32;
//         // }
//         // if celeste.mem.buttons[1] {
//         //     self.pos.x += 1f32;
//         // }
//         // self
//     }
//     fn draw(&mut self, celeste: &mut Celeste) {
//         let clamped = mid(self.pos.x, -1.0, 121.0);
//         if self.pos.x != clamped {
//             self.pos.x = clamped;
//             self.spd.x = 0.0;
//         }

//         // self.spr += 1;
//         let haircol = if self.djump == 1 { 8 } else { 12 };
//         celeste.mem.pal(8, haircol);

//         let mut first = Vector {
//             x: self.pos.x + if self.flip.x { 6.0 } else { 2.0 },
//             y: self.pos.y + if celeste.mem.buttons[3] { 4.0 } else { 3.0 },
//         };
//         // celeste.mem.circfill(50, 50, 2.into(), 3);
//         let mut last = &mut first;
//         for (i, h) in self.hair.iter_mut().enumerate() {
//             h.x += (last.x - h.x) / 1.5;
//             h.y += (last.y + 0.5 - h.y) / 1.5;
//             celeste
//                 .mem
//                 .circfill(h.x as u8, h.y as u8, (3 - i).min(2).max(1) as i8, haircol);
//             last = h;
//         }
//         self.draw_sprite(celeste);
//         celeste.mem.pal(8, 8);
//     }

//     // "fields"
//     // yeah, really stupid but its a workaround for traits not having fields
//     // reaching java levels of boilerplate here, remember i need to do this for every object lmao
//     // plus the borrow checkers gonna complain as soon as i want to do literally anything
//     // unless i refactor to use refcells but :\
//     fn pos(&self) -> &Vector {
//         &self.pos
//     }
//     fn spd(&self) -> &Vector {
//         &self.spd
//     }
//     fn rem(&self) -> &Vector {
//         &self.rem
//     }
//     fn spr(&self) -> &u8 {
//         &self.spr
//     }
//     fn hitbox(&self) -> &Rectangle {
//         &self.hitbox
//     }
//     fn flip(&self) -> &FlipState {
//         &self.flip
//     }

//     fn pos_mut(&mut self) -> &mut Vector {
//         &mut self.pos
//     }
//     fn spd_mut(&mut self) -> &mut Vector {
//         &mut self.spd
//     }
//     fn rem_mut(&mut self) -> &mut Vector {
//         &mut self.rem
//     }
//     fn spr_mut(&mut self) -> &mut u8 {
//         &mut self.spr
//     }
//     fn collidable(&self) -> &bool {
//         &self.collidable
//     }
//     fn name(&self) -> &'static str {
//         self.name
//     }
// }

// pub struct Balloon {
//     pub pos: Vector,
//     pub spd: Vector,
//     pub rem: Vector,
//     pub spr: u8,
//     offset: f32,
// }
// impl Object for Balloon {
//     fn init(celeste: &mut Celeste, x: f32, y: f32) -> Self
//     where
//         Self: Sized,
//     {
//         Balloon {
//             pos: Vector { x, y },
//             rem: Vector { x: 0f32, y: 0f32 },
//             spd: Vector { x: 0f32, y: 0f32 },
//             spr: 1,
//             offset: celeste.mem.rng.gen_range(0.0..1.0),
//         }
//     }
//     fn update(&mut self, celeste: &mut Celeste) {}
//     fn draw(&mut self, celeste: &mut Celeste) {
//         if self.spr == 22 {
//             celeste.mem.spr(
//                 (13.0 + (self.offset * 8.0) % 3.0) as u8,
//                 self.pos.x as i32,
//                 self.pos.y as i32,
//                 None,
//             );
//             self.draw_sprite(celeste);
//         }
//     }

//     fn pos(&self) -> &Vector {
//         &self.pos
//     }
//     fn spd(&self) -> &Vector {
//         &self.spd
//     }
//     fn rem(&self) -> &Vector {
//         &self.rem
//     }
//     fn spr(&self) -> &u8 {
//         &self.spr
//     }
//     fn hitbox(&self) -> &Rectangle {
//         &Rectangle {
//             x: -1.0,
//             y: -1.0,
//             w: 10.0,
//             h: 10.0,
//         }
//     }
//     fn flip(&self) -> &FlipState {
//         &FlipState { x: false, y: false }
//     }
//     fn collidable(&self) -> &bool {
//         &false
//     }
//     fn name(&self) -> &'static str {
//         "balloon"
//     }

//     fn pos_mut(&mut self) -> &mut Vector {
//         &mut self.pos
//     }
//     fn spd_mut(&mut self) -> &mut Vector {
//         &mut self.spd
//     }
//     fn rem_mut(&mut self) -> &mut Vector {
//         &mut self.rem
//     }
//     fn spr_mut(&mut self) -> &mut u8 {
//         &mut self.spr
//     }
// }
