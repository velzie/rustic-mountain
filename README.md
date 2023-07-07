## Rustic-Mountain

the pico-8 classic you all know and love, now with fearless concurrency

inspired by ccleste and pyleste

ported from [smalleste.p8](https://github.com/CelesteClassic/smalleste/blob/main/smalleste.p8)

### librustic
a headless API is exposed through the "core" crate, so you can import it into any rust project, or compile to a shared object for FFI use in any programming language<br>
this feature is used in [morespriteshorn](https://github.com/CoolElectronics/morespriteshorn) to simulate accurate celeste physics inside of the love2d engine

# DISCLAIMER:

despite my best efforts, this is not a 100% faithful port. expect some of the more subtle quirks of celeste to not work as expected

since the goal was to achieve a 1:1 translation of the lua code, this isn't best practice rust and is full of things that should probably not be done

special thanks to the [celeste classic discord server](https://discord.gg/9Dm3NCS)
