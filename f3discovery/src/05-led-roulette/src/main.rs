#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};
use volatile::Volatile;

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let mut period = 150_u16;
    let v_period = Volatile::new(&mut period);

    // infinite loop; just so we don't leave this stack frame
    loop {
        for i in 0..8 {
            let next = (i + 1) % 8;
            // (1+1) % 8 = 2 % 8 = 2
            // (2+1) % 8 = 3 % 8 = 3
            // (8+1) % 8 = 9 % 8 = 1 <= modulo stops from accessing past 8 element

            leds[next].on().ok();
            delay.delay_ms(v_period.read());

            leds[i].off().ok();
            delay.delay_ms(v_period.read());
        }
    }
}
