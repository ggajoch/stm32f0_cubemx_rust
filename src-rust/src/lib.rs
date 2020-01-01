#![crate_type = "staticlib"]
#![feature(lang_items)]
#![no_std]
#![no_builtins]

use panic_semihosting as _;

extern "C" {
    fn cxx_led_on();
    fn cxx_led_off();
    fn cxx_delay(ms: u32);
}

fn led_on() {
    unsafe { cxx_led_on(); }
}
fn led_off() {
    unsafe { cxx_led_off(); }
}
fn delay(ms: u32) {
    unsafe { cxx_delay(ms); }
}

#[no_mangle]
pub extern "C" fn rust_main() {
    loop {
        led_on(); 
        delay(100);
        led_off();
        delay(200);
    }
}