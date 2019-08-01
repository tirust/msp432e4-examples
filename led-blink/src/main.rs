#![no_std]
#![no_main]


#[macro_use]
extern crate lazy_static;
extern crate panic_halt;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::{entry, exception};
use cortex_m::interrupt::{self, Mutex};

lazy_static! {
    static ref LED_GPIO: Mutex<msp432e4::GPION> = {
        let mut _periph = msp432e4::Peripherals::take().unwrap();
        initialize_power(&_periph.SYSCTL);
        initialize_gpion(&_periph.GPION);
        Mutex::new(_periph.GPION)
    };
}


#[entry]
fn main() -> ! {
    let cm_p = cortex_m::Peripherals::take().unwrap();    

    /* Setup the SysTick */
    let mut systick = cm_p.SYST;
   
    // configure systick interrupt each second 
    systick.set_clock_source(SystClkSource::Core);
    // 12 MHz setting
    systick.set_reload(1_200_000);
    systick.enable_counter();
    systick.enable_interrupt();

    loop {
        /* Do nothing, operating in interrupts */
    }
}

fn initialize_power(syscfg: &msp432e4::SYSCTL) {
    syscfg.rcgcgpio.write(|w| w.sysctl_rcgcgpio_r12().bit(true));
}

fn initialize_gpion(gpion: &msp432e4::GPION) {
    gpion.dir.modify(|r, w| unsafe { w.bits(r.bits() | 0xFF) });
    gpion.den.modify(|r, w| unsafe { w.bits(r.bits() | 0xFF) });
}

#[exception]
fn SysTick() {
    interrupt::free( |cs| {
        let gpion = LED_GPIO.borrow(cs);
        gpion.data.modify(|r, w| unsafe { w.bits(r.bits() ^ 0xFF) });
    }); 
}
