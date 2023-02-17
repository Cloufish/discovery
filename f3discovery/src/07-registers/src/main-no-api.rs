#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprintln, RegisterBlock, ITM};

// Prints the current contents of odr
fn iprint_odr(itm: &mut ITM) {
    const GPIOE_ODR: u32 = 0x4800_1014;

    unsafe {
        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *const u16)
        );
    }
}

#[entry]
fn main() -> ! {
    let mut itm = aux7::init().0;

    unsafe {
        // A magic address!
        const GPIOE_BSRR: u32 = 0x4800_1018; // To register that controls GPIO pins

        // Print the initital contents of ODR:
        iprint_odr(&mut itm);

        // '<<' is a shift operation:

        // We need to make writing to the register Volatile becase of mis-optimizations
        // - The compiler's backend doesn't know we're dealing with a register ..
        // and will merge the writes
        // - The optimizations won't be performed if we compile it with debug flag
        //

        // Turn on the "North" LED (red)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);
        // 00000000 00000001 changes to 00000010 00000000
        iprint_odr(&mut itm);

        // Turn on the "East" LED (green)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);
        iprint_odr(&mut itm);

        // Turn off the "North" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));
        iprint_odr(&mut itm);

        // Turn off the "East" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));
        iprint_odr(&mut itm);
    }

    loop {}
}
