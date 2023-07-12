use rustic_mountain_core::*;
use std::time::Duration;

use std::io::{self, Write};
use termion::{
    event::{Event, Key},
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    terminal_size,
};
mod consts;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = Celeste::new(
        consts::MAPDATA.into(),
        consts::SPRITES.into(),
        consts::FLAGS.into(),
        consts::FONTATLAS.into(),
    );
    engine.load_room(0, 0);
    engine.level = 0;
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

    let size = terminal_size()?;
    if size.0 < 128 || size.1 < 64 {
        panic!("your terminal is too small! change the font size so there's at least 128 rows and 64 lines");
    }

    let term = io::stdout().into_raw_mode()?;
    term.activate_raw_mode()?;

    let timings: &mut [u8; 4] = &mut [0, 0, 0, 0];

    let mut stdout = MouseTerminal::from(term.lock());

    let stdin = termion::async_stdin();
    let mut events = stdin.events();

    write!(stdout, "{}", termion::clear::All)?;

    let mut savestate: Option<String> = None;
    loop {
        engine.next_tick();
        engine.draw();

        let mut buffer = String::new();
        for (i, col) in engine.mem.graphics.iter().enumerate() {
            if (i / 128) % 2 == 1 {
                continue;
            }
            let c = pallete[*col as usize];
            buffer += &format!(
                "{} ",
                termion::color::Bg(termion::color::Rgb(c.0, c.1, c.2))
            );
            if i % 128 == 0 {
                buffer += &format!("{}", termion::cursor::Goto(1, i as u16 / 256 + 1));
            }
        }

        write!(stdout, "{}", termion::cursor::Goto(1, 1))?;
        write!(stdout, "{}", buffer)?;
        stdout.flush()?;

        for (i, t) in timings.iter_mut().enumerate() {
            if *t <= 0 {
                engine.mem.buttons[i] = false;
            } else {
                *t -= 1;
            }
        }

        let key_duration = 10;

        engine.mem.buttons[4] = false;
        engine.mem.buttons[5] = false;

        if let Some(Ok(e)) = events.next() {
            match e {
                Event::Key(k) => match k {
                    Key::Left => {
                        engine.mem.buttons[0] = true;
                        timings[0] = key_duration;
                    }
                    Key::Right => {
                        engine.mem.buttons[1] = true;
                        timings[1] = key_duration;
                    }
                    Key::Up => {
                        engine.mem.buttons[2] = true;

                        timings[2] = key_duration;
                    }
                    Key::Down => {
                        timings[3] = key_duration;
                        engine.mem.buttons[3] = true;
                    }
                    Key::Char('e') => match engine.save_state() {
                        Ok(e) => savestate = Some(e),
                        Err(e) => panic!("{:?}", e),
                    },

                    Key::Char('f') => {
                        engine.next_room();
                    }
                    Key::Char('q') => {
                        if let Some(s) = &savestate {
                            engine.load_state(s);
                        }
                    }
                    Key::Char('z') => {
                        engine.mem.buttons[4] = true;
                    }
                    Key::Char('c') => {
                        engine.mem.buttons[4] = true;
                    }
                    Key::Char('x') => {
                        engine.mem.buttons[5] = true;
                    }
                    Key::Ctrl('c') => {
                        panic!();
                    }
                    _ => (),
                },
                _ => (),
            }
        };

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
