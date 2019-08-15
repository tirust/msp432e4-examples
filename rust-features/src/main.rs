#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use msp432e4;

mod uart;

#[derive(Clone)]
struct Temperature {
    time: u32,
    temp: u32
}

struct TemperatureReader<'a> {
    uart: &'a msp432e4::UART0,
    buf: [u8; 80]
}

impl<'a> Iterator for TemperatureReader<'a> {
    type Item = Temperature;

    fn next(&mut self) -> Option<Temperature> {
        uart::uart_receive_string(&self.uart, &mut self.buf);
        let mut time_a: [u8; 4] = [0, 0, 0, 0];
        let mut temp_a: [u8; 4] = [0, 0, 0, 0];
        /* 
         * Using 32-bit temperature and 32-bit time,
         * expect 8 bytes
         */
        time_a.copy_from_slice(&self.buf[0..4]);
        temp_a.copy_from_slice(&self.buf[4..8]);
        Some(
            Temperature {
                    time: unsafe { core::mem::transmute_copy::<[u8; 4], u32>(&time_a) },
                    temp: unsafe { core::mem::transmute_copy::<[u8; 4], u32>(&temp_a) }
            }
        )
    }
}

fn temperature_reader(uart: &msp432e4::UART0) -> TemperatureReader {
    TemperatureReader {
        uart,
        buf: [0; 80]
    }
}

#[entry]
fn main() -> ! {
   
    let peripherals = msp432e4::Peripherals::take().unwrap();
 
    uart::initialize_uart(&peripherals.SYSCTL, &peripherals.GPIOA, &peripherals.UART0);
    uart::uart_transmit_string(&peripherals.UART0, "Welcome to temperature monitor");    
    /* Pass ownership of device peripherals to iterator */
    loop {
        let iter = temperature_reader(&peripherals.UART0);
        iter.for_each(|x| {
            let air_temp = x.temp / 1000;
            let hour = x.time / 3600;
            let min = (x.time % 3600) / 60;
            let sec = x.time % 60;
            hprintln!("{}:{}:{} - {} Degrees F", hour, min, sec, air_temp).unwrap();
        });
    }
}
