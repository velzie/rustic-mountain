use rustic_mountain_core::Celeste;
use wasm_bindgen::prelude::*;
mod test;
///...
/// belive me, i tried
/// i tried lazy_static, wrapping mutexes in rwlocks in boxes, experimental compiler features, passing closures as JsValues, even more stuff
/// there's just not a safe way of having a global ownerless mutable singleton that can be written to concurrently
/// or maybe i'm just stupid
/// anyway have this raw pointer lol segfault
static mut CELESTE: *mut Celeste = std::ptr::null_mut::<Celeste>();

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn start(map: String, sprites: String, flags: String, fontatlas: String) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    unsafe {
        console_error_panic_hook::set_once();
        CELESTE = Box::leak(Box::new(Celeste::new(map, sprites, flags, fontatlas)));
        (*CELESTE).mem.logger = Box::new(log);
        // (*CELESTE).mem
        // ^ intentionally causes a memory leak so that CELESTE will stay around for the entire time the WASM is loaded in memory. if you have a better way of doing this let me know i guess
        log("initialized celeste");
    }
}

#[wasm_bindgen]
pub fn next_tick() {
    unsafe {
        (*CELESTE).next_tick();
    }
}
#[wasm_bindgen]
pub fn render_screen() -> Vec<u8> {
    unsafe {
        (*CELESTE).draw();
        (*CELESTE).mem.graphics.clone()
    }
}
#[wasm_bindgen]
pub fn skip_level(){
    unsafe{
        (*CELESTE).next_room();
    }
}

#[wasm_bindgen]
pub fn set_btn(btn: u8, val: bool) {
    unsafe {
        // log(&format!("btn {} set to {}", btn, val));
        (*CELESTE).mem.buttons[btn as usize] = val;
    }
}
