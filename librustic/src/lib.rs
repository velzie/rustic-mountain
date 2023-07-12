#![feature(vec_into_raw_parts)]

use std::{
    any,
    ffi::{c_char, c_void, CStr},
};

use rustic_mountain_core::Celeste;

/// raw pointer neccesary
/// shouldn't break, just be careful with it
static mut CELESTE: *mut Celeste = std::ptr::null_mut::<Celeste>();

static mut SAVESTATE: *mut Option<String> = std::ptr::null_mut();
#[no_mangle]
pub extern "C" fn librustic_start(
    map: *const c_char,
    sprites: *const c_char,
    flags: *const c_char,
    fontatlas: *const c_char,
) {
    unsafe {
        let none = Box::leak(Box::new(None));
        SAVESTATE = none;
        CELESTE = Box::leak(Box::new(Celeste::new(
            CStr::from_ptr(map).to_string_lossy().to_string(),
            CStr::from_ptr(sprites).to_string_lossy().to_string(),
            CStr::from_ptr(flags).to_string_lossy().to_string(),
            CStr::from_ptr(fontatlas).to_string_lossy().to_string(),
        )));
    }
}

#[no_mangle]
pub extern "C" fn librustic_test() -> u8 {
    42
}

#[no_mangle]
pub extern "C" fn librustic_next_tick() {
    unsafe {
        (*CELESTE).next_tick();
    }
}
#[no_mangle]
pub extern "C" fn librustic_save_state() {
    unsafe {
        let res = (*CELESTE).save_state();
        if let Ok(s) = res {
            *SAVESTATE = Some(s);
        }
    }
}
#[no_mangle]
pub extern "C" fn librustic_load_state() {
    unsafe {
        if let Some(state) = &(*SAVESTATE) {
            (*CELESTE).load_state(state);
        }
    }
}

#[repr(C)]
pub struct FFIVec {
    ptr: *mut c_void,
    length: usize,
}

#[no_mangle]
pub extern "C" fn librustic_render_screen() -> *mut c_void {
    unsafe {
        (*CELESTE).draw();
        let vec = (*CELESTE).mem.graphics.clone();
        let boxed = vec.into_boxed_slice();
        Box::into_raw(boxed) as *mut c_void
    }
}
#[no_mangle]
pub extern "C" fn librustic_skip_level() {
    unsafe {
        (*CELESTE).next_room();
    }
}

#[no_mangle]
pub extern "C" fn librustic_set_btn(btn: u8, val: bool) {
    unsafe {
        // log(&format!("btn {} set to {}", btn, val));
        (*CELESTE).mem.buttons[btn as usize] = val;
    }
}
