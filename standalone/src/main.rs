use rustic_mountain_core::*;
use structures::*;
mod consts;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

static SCALE: u8 = 5;

pub fn main() {
    let mut engine = Celeste::new(
        consts::MAPDATA.into(),
        consts::SPRITES.into(),
        consts::FLAGS.into(),
        consts::FONTATLAS.into(),
    );
    let pallete: [(u8, u8, u8); 16] = [
        (0, 0, 0),
        (29, 43, 83),
        (126, 37, 83),
        (0, 135, 81),
        (171, 82, 54),
        (95, 87, 79),
        (194, 195, 199),
        (255, 241, 232),
        (255, 0, 77),
        (255, 163, 0),
        (255, 236, 85),
        (0, 228, 54),
        (41, 173, 255),
        (131, 118, 156),
        (255, 119, 168),
        (255, 204, 170),
    ];
    // rustic_mountain_core::structures::
    // engine.objects.push(structures::Object {
    //     pos: Vector::new(0, 0),
    //     obj: &structures::Player {},
    // });

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rustic-mountain", 128 * SCALE as u32, 128 * SCALE as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // canvas.set_draw_color(Color::BLUE);
        // canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(code),
                    ..
                } => match code {
                    Keycode::Left => {
                        engine.mem.buttons[0] = true;
                    }
                    Keycode::Right => {
                        engine.mem.buttons[1] = true;
                    }
                    Keycode::Up => {
                        engine.mem.buttons[2] = true;
                    }
                    Keycode::Down => {
                        engine.mem.buttons[3] = true;
                    }
                    Keycode::Z => {
                        engine.mem.buttons[4] = true;
                    }
                    Keycode::X => {
                        engine.mem.buttons[5] = true;
                    }
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(code),
                    ..
                } => match code {
                    Keycode::Left => {
                        engine.mem.buttons[0] = false;
                    }
                    Keycode::Right => {
                        engine.mem.buttons[1] = false;
                    }
                    Keycode::Up => {
                        engine.mem.buttons[2] = false;
                    }
                    Keycode::Down => {
                        engine.mem.buttons[3] = false;
                    }
                    Keycode::Z => {
                        engine.mem.buttons[4] = false;
                    }
                    Keycode::X => {
                        engine.mem.buttons[5] = false;
                    }
                    Keycode::F => {
                        engine.next_room();
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        engine.next_tick();
        engine.draw();

        for (i, col) in engine.mem.graphics.iter().enumerate() {
            canvas.set_draw_color(pallete[*col as usize]);
            canvas
                .fill_rect(sdl2::rect::Rect::new(
                    (i % 128 * SCALE as usize) as i32,
                    (i / 128 * SCALE as usize) as i32,
                    SCALE as u32,
                    SCALE as u32,
                ))
                .unwrap();
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
