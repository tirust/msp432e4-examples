#![no_std]
#![no_main]

#[macro_use]
extern crate lazy_static;
extern crate panic_halt;

use cortex_m_rt::entry;
use msp432e4::{Interrupt, interrupt};

mod comp;

lazy_static! {
    static ref PERIPHERALS: cortex_m::interrupt::Mutex<msp432e4::Peripherals> = {
        let peripherals = msp432e4::Peripherals::take().unwrap();
        cortex_m::interrupt::Mutex::new(peripherals)
    };
}

#[entry]
fn main() -> ! {
    let mut core = cortex_m::Peripherals::take().unwrap();

    cortex_m::interrupt::free(|cs| {
        let peripherals = PERIPHERALS.borrow(cs);
        comp::setup_comparator(&peripherals);
        comp::setup_gpio(&peripherals);
        peripherals.SYSCTL.rcgcgpio.write(|w| w.sysctl_rcgcgpio_r12().bit(true));
        // configure as digital output
        peripherals.GPION.dir.modify(|r, w| unsafe { w.bits(r.bits() | 0xFF) });
        peripherals.GPION.den.modify(|r, w| unsafe { w.bits(r.bits() | 0xFF) });
    });
 
    core.NVIC.enable(Interrupt::COMP0);

    loop {
        // Nothing, operating in interrupts only
    }
}

#[interrupt]
fn COMP0() {
    // clear interrupt pending status
    cortex_m::interrupt::free(|_cs| {
        cortex_m::peripheral::NVIC::unpend(Interrupt::COMP0);
    });

    // get comparator value
    let mut comp_out = 0xFF;

    cortex_m::interrupt::free(|cs| {
        let peripherals = PERIPHERALS.borrow(cs);
        peripherals.COMP.acmis.write(|w| {
            unsafe { w.bits(0x01) }
        });
        comp_out = comp::read_comparator(&peripherals.COMP);
    });

    cortex_m::interrupt::free(|cs| {
        let peripherals = PERIPHERALS.borrow(cs);
        if comp_out != 0 {
            peripherals.GPION.data.modify(|r, w| unsafe { w.bits(r.bits() | 0xFF) });
        } else {
            peripherals.GPION.data.modify(|r, w| unsafe { w.bits(r.bits() & !0xFF) });
        }
    });
}
