#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

mod uart;


#[entry]
fn main() -> ! {
    /* Take Cortex-M peripherals */
    let _core = cortex_m::Peripherals::take().unwrap();
    /* Take device peripherals */
    let peripherals: msp432e4::Peripherals = msp432e4::Peripherals::take().unwrap();

    /* Create receive buffer */
    let mut buf: [u8; 80] = [0; 80];
    /* Initialize UART device */
    uart::initialize_uart(&peripherals.SYSCTL, &peripherals.GPIOA, &peripherals.UART0);
    /* Initialize LED */
    uart::initialize_led(&peripherals.SYSCTL, &peripherals.GPION);
    
    uart::uart_transmit_string(&peripherals.UART0, "Welcome, type a command.\r\n");

    loop {
        /* Blocks for user input */
        uart::uart_receive_string(&peripherals.UART0, &mut buf);
        /* Process command */
        let state = core::str::from_utf8(&buf).unwrap();
        if state.contains("LED ON") {
            uart::led_write(&peripherals.GPION, uart::LedState::On);
            uart::uart_transmit_string(&peripherals.UART0, "Lights are on\r\n");
        } else if state.contains("LED OFF") {
            uart::led_write(&peripherals.GPION, uart::LedState::Off);
            uart::uart_transmit_string(&peripherals.UART0, "Lights out!\r\n");
        } else {
            uart::uart_transmit_string(&peripherals.UART0, "I only know about lights.\r\n");
        }
    }
}
