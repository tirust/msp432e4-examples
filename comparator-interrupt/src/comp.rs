extern crate panic_halt;

pub fn setup_comparator(peripherals: &msp432e4::Peripherals) {
    peripherals.SYSCTL.rcgcacmp.write(|w| w.sysctl_rcgcacmp_r0().bit(true));
    // set up control register for COMP0 
    peripherals.COMP.acctl0.write(|w| {
        w.comp_acctl0_cinv().bit(true);
        w.comp_acctl0_isen().bits(0b11);
        unsafe { w.comp_acctl0_asrcp().bits(0b10) }
    });
    // set up interrupts for COMP0
    peripherals.COMP.acinten.write(|w| {
        w.comp_acinten_in0().bit(true)
    });
    // setup reference voltage
    peripherals.COMP.acrefctl.write(|w| {
        unsafe { w.comp_acrefctl_vref().bits(0x8); }
        w.comp_acrefctl_rng().bit(true);
        w.comp_acrefctl_en().bit(true)
    });
}

pub fn setup_gpio(peripherals: &msp432e4::Peripherals) {
    // enable power to GPIOC
    peripherals.SYSCTL.rcgcgpio.write(|w| w.sysctl_rcgcgpio_r2().bit(true));
    // setup PC7 as Analog Comparator input
    unsafe {
        peripherals.GPIOC.afsel.write(|w| w.bits(0));
        peripherals.GPIOC.dir.write(|w| w.bits(0));
        peripherals.GPIOC.den.write(|w| w.bits(0));
    }
    // setup PD0 as Analog Comparator output
    unsafe {
        peripherals.GPIOD.afsel.write(|w| w.bits(0b1));
        peripherals.GPIOD.den.write(|w| w.bits(0b1));
    }
}

pub fn read_comparator(comparator: &msp432e4::COMP) -> u32 {
    comparator.acstat0.read().bits()
}
