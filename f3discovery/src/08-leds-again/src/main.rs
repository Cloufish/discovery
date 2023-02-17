#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux8::entry;

#[entry]
fn main() -> ! {
    let (gpioe, rcc) = aux8::init();

    // TODO initialize GPIOE

    //1.Figure out which of the three registers
    //I mentioned before has the bit that controls the power status.

    //2. Figure out what value that bit must be set to,
    //    0  or 1 , to power on the GPIOE  peripheral.

    //3.Finally, you'll have to change the starter code to
    //modify the right register to turn on the GPIOE

    // Turn on all the LEDs in the compass
    gpioe.odr.write(|w| {
        w.odr8().set_bit();
        w.odr9().set_bit();
        w.odr10().set_bit();
        w.odr11().set_bit();
        w.odr12().set_bit();
        w.odr13().set_bit();
        w.odr14().set_bit();
        w.odr15().set_bit()
    });

    aux8::bkpt();

    loop {}
}
