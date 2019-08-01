use cortex_m::asm;

pub enum LedState {
    On,
    Off,
}

pub fn led_write(gpion: &msp432e4::GPION, state: LedState) {
    match state {
        LedState::On => gpion.data.modify(|r, w| unsafe { w.bits(r.bits() | 0xFF) }),
        LedState::Off => gpion.data.modify(|r, w| unsafe { w.bits(r.bits() & !0xFF) })
    }
}

pub fn initialize_led(sysctl: &msp432e4::SYSCTL, gpion: &msp432e4::GPION) {
    /* Enable clock and power */
    sysctl.rcgcgpio.write(|w| {
        w.sysctl_rcgcgpio_r12().bit(true)
    });

    /* Setup digital output */
    gpion.dir.modify(|r, w| unsafe { w.bits(r.bits() | 0xFF) });
    gpion.den.modify(|r, w| unsafe { w.bits(r.bits() | 0xFF) });
}

pub fn uart_receive_string(uart: &msp432e4::UART0, buf: &mut [u8; 80]) {
    let mut pos = 0;

    loop {
        while !uart.fr.read().uart_fr_rxff().bit() {
            asm::nop();
        }
        
        let byte = uart.dr.read().uart_dr_data().bits();

        if byte == '\n' as u8 || byte == '\r' as u8 {
            break;
        } else {
            buf[pos] = byte;
            pos += 1;
            if pos == 80 {
                break;
            }
        }
    }

}

pub fn uart_transmit_string(uart: &msp432e4::UART0, msg: &str) {
    for byte in msg.bytes() {
        while uart.fr.read().uart_fr_busy().bit() {
            asm::nop();
        }
        uart.dr.write(|w| unsafe { w.uart_dr_data().bits(byte) });
    } 
}


pub fn initialize_uart(sysctl: &msp432e4::SYSCTL, gpioa: &msp432e4::GPIOA, uart: &msp432e4::UART0) {
    /* Enable power to UART0 module */
    sysctl.rcgcuart.write(|w| {
        w.sysctl_rcgcuart_r0().bit(true)
    });

    /* Enable power to GPIOA module */
    sysctl.rcgcgpio.write(|w| {
        w.sysctl_rcgcgpio_r0().bit(true)
    });

    /* Require 3 cycles between peripheral enable and access */
    asm::nop();
    asm::nop();
    asm::nop();

    /* Set up GPIO PA0 & PA1 for UART TX/RX operation */
    /* Note: "unsafe" block required here due to SVD file not specifying bit fields */
    gpioa.afsel.modify(|r, w| {
        unsafe { w.bits(r.bits() | 0b11) }
    });

    gpioa.den.modify(|r, w| {
        unsafe { w.bits(r.bits() | 0b11) }
    });

    gpioa.odr.modify(|r, w| {
        unsafe { w.bits(r.bits() & !0b11) }
    });
    
    /* Set pin mux */
    gpioa.pctl.write(|w| {
        unsafe { w.bits(0x11) }
    });
    
    /* Set up UART0 module 
     *  > Baud rate: 115200
     *  > 8 bits, single stop bit, no parity
     *  > No FIFO
     */
    /* Ensure module is disabled for configuration */
    uart.ctl.modify(|_, w| w.uart_ctl_uarten().bit(false));
    /* Set baud rate divisor */
    uart.ibrd.write(|w| unsafe { w.uart_ibrd_divint().bits(8) });
    uart.fbrd.write(|w| unsafe { w.uart_fbrd_divfrac().bits(44) });
    /* Serial parameters */
    uart.lcrh.write(|w| { w.uart_lcrh_wlen().bits(0x3) });
    /* Enable UART module */
    uart.ctl.modify(|_, w| w.uart_ctl_uarten().bit(true));
     
}
