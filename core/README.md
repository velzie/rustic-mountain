# Rustic Mountain Core
A rust library for emulating the physics and graphics of the 2015 game jam release of Celeste Classic

# Install
```
cargo add rustic-mountain-core
```

# Basic Usage (eg, creating a port)
```
fn main(){
    // consts are not included in the core library. see examples here: https://github.com/CoolElectronics/rustic-mountain/blob/main/standalone/src/consts.rs
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

    loop {
        // advance the game logic
        engine.next_tick();
        // render the screen
        engine.draw();



        // screen buffer is a 128x128 array
        for (i, col) in engine.mem.graphics.iter().enumerate() {
            // look up rgb color from pallete
            let color = pallete[*col as usize];
            let xpixel = i % 128;
            let ypixel = i / 128;

            // do rendering to screen here
        }

                        engine.mem.buttons[0] = is_left_arrow_pressed;
                        engine.mem.buttons[1] = is_right_arrow_pressed;
                        engine.mem.buttons[2] = is_up_arrow_pressed;
                        engine.mem.buttons[3] = is_down_arrow_pressed;
                        engine.mem.buttons[4] = is_jump_pressed;
                        engine.mem.buttons[5] = is_dash_key_pressed;


        // constrain to 30fps
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}
```


# Advanced usage
Most methods and fields are marked public, so the game can be messed with and extended easily. For example, you can iterate over the `celeste.objects` vector to find the player position, create your own maps, etc, etc. I'm not making docs, just read the code the main parts is only a little over 1k lines


