use rustic_mountain_core::Celeste;
use wasm_bindgen::prelude::*;
mod test;

/// raw pointer neccesary in wasm
/// shouldn't break, just be careful with it
static mut CELESTE: *mut Celeste = std::ptr::null_mut::<Celeste>();

static mut SAVESTATE: *mut Option<String> = std::ptr::null_mut();

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

        let none = Box::leak(Box::new(None));
        SAVESTATE = none;
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
pub fn save_state() {
    unsafe {
        let res = (*CELESTE).save_state();
        if let Ok(s) = res {
            *SAVESTATE = Some(s);
        } else {
            log("???")
        }
    }
}
#[wasm_bindgen]
pub fn load_state() {
    unsafe {
        if let Some(state) = &(*SAVESTATE) {
            (*CELESTE).load_state(state);
        }
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
pub fn skip_level() {
    unsafe {
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
