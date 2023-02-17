#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprintln, RegisterBlock, ITM};

#[entry]
fn main() -> ! {
    let gpioe = aux7::init().1;

    // Turning off/on LED has the same methor set_bit()

    gpioe.bsrr.write(|w| w.bs9().set_bit());

    gpioe.bsrr.write(|w| w.bs11().set_bit());

    gpioe.bsrr.write(|w| w.bs9().set_bit());

    gpioe.bsrr.write(|w| w.bs11().set_bit());

    loop {}
}
