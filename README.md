the pico-8 classic you all know and love, now with fearless concurrency

Rustic Mountain
===============
[<img alt="github" src="https://img.shields.io/badge/github-coolelectronics/rustic-mountain?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/CoolElectronics/rustic-mountain)
[<img alt="crates.io" src="https://img.shields.io/crates/v/rustic-mountain-core.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/rustic-mountain-core)

A reimplementation of Celeste Classic in rust, with ports to windows and linux, android and the web
### official ports
- [web demo](https://coolelectronics.me/rustic-demo)
- [PC standalone](https://github.com/CoolElectronics/rustic-mountain/releases)
- [Android APK](https://github.com/CoolElectronics/rustic-mountain/releases)
- [crates.io package](https://crates.io/crates/rustic-mountain-core)
### other ports
- [Apple Watch project](https://github.com/r58Playz/watchleste)

to compile standalone:
```
git clone https://github.com/CoolElectronics/rustic-mountain
cd rustic-mountain
cargo build
```

to compile for web:
```
cargo install wasm-pack
git clone https://github.com/CoolElectronics/rustic-mountain
cd rustic-mountain/web
wasm-pack build --target web
```
to compile for android:
```
cargo install cross
git clone https://github.com/CoolElectronics/rustic-mountain
cd rustic-mountain/librustic
cross build --target aarch64-linux-android --release
cp ../target/aarch64-linux-android/release/librustic_mountain_lib.so ../android/app/libs/arm64-v8a/
cp ../target/aarch64-linux-android/release/librustic_mountain_lib.so ../android/app/libs/armeabi/
cp ../target/aarch64-linux-android/release/librustic_mountain_lib.so ../android/app/libs/jniLibs/

# open android studio and build normally, or do it with gradle idk
```


### tuileste
![image](https://github.com/CoolElectronics/rustic-mountain/assets/58010778/ff23acd8-3b88-4642-abc5-71af126a3a77)
this is an experimental port with to the linux terminal. to use it:
```
git clone https://github.com/CoolElectronics/rustic-mountain
cd rustic-mountain/tuileste
cargo run
```
the controls are extremely scuffed because of the limitations of terminals. use a modern terminal like kitty or konsole. pressing any of the arrow keys will hold them down for 10 frames, so you'll have to sort of press and unpress it every 10 frames if you want to hold it down. have fun

as usual, z+x for jump and dash, arrow keys to move, f to skip level, 'q' to make a savestate, 'e' to load a savestate

### librustic
a headless API is exposed through the "core" crate, so you can import it into any rust project, or compile to a shared object for FFI use in any programming language<br>
this feature is used in [morespriteshorn](https://github.com/CoolElectronics/morespriteshorn) to simulate celeste physics inside of the love2d engine

# DISCLAIMER:

despite my best efforts, this is not a 100% faithful port. expect some of the more subtle quirks of celeste to not work as expected

since the goal was to achieve a 1:1 translation of the lua code, this isn't best practice rust and is full of things that should probably not be done

inspired by ccleste and pyleste

ported from [smalleste.p8](https://github.com/CelesteClassic/smalleste/blob/main/smalleste.p8)

special thanks to the [celeste classic discord server](https://discord.gg/9Dm3NCS)

