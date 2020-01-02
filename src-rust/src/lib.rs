#![cfg_attr(not(test), no_std)]
#![feature(proc_macro_hygiene)]

#[cfg(not(test))]
use panic_semihosting as _;

extern "C" {
    fn cxx_led_on();
    fn cxx_led_off();
    fn cxx_delay(ms: u32);
}

mod wrapper {
    cfg_if::cfg_if! {
        if #[cfg(test)] {
            use mockall::automock;

        }
    }

    #[cfg_attr(test, automock())]
    pub(super) mod hal {

        pub fn led_on() {
            unsafe { crate::cxx_led_on(); }
        }

        pub fn led_off() {
            unsafe { crate::cxx_led_off(); }
        }

        pub fn delay(ms: u32) {
            unsafe { crate::cxx_delay(ms); }
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(test)] {
        use wrapper::mock_hal as hal;
    } else {
        use wrapper::hal;
    }
}

fn loop_iteration() {
    hal::led_on();
    hal::delay(100);
    hal::led_off();
    hal::delay(200);
}

#[no_mangle]
pub extern "C" fn rust_main() {
    loop {
        loop_iteration();
    }
}





#[cfg(test)]
mod tests {
    use mockall::*;
    use mockall::predicate::*;
    use crate::loop_iteration;
    use crate::hal;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


    #[test]
    fn loop_turns_on() {
        let mut seq = Sequence::new();

        let led_on_ctx = hal::led_on_context();
        let led_off_ctx = hal::led_off_context();
        let delay_ctx = hal::delay_context();

        led_on_ctx.expect().times(1).returning(|| ()).in_sequence(&mut seq);
        delay_ctx.expect().with(eq(100)).times(1).in_sequence(&mut seq);
        led_off_ctx.expect().times(1).in_sequence(&mut seq);
        delay_ctx.expect().with(eq(200)).times(1).in_sequence(&mut seq);

        loop_iteration();
    }
}
