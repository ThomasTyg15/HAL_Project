#![no_std]
#![no_main]

use core::panic::PanicInfo;
use my_hal::{
    hal::Target,
    gpio::{AtmegaPin, CortexPin, Direction},
    usart::{AtmegaUsart, CortexUsart},
    drivers::{Led, Serial},
};

#[cfg(feature = "atmega328p")]
use avr_device::atmega328p::Peripherals as AtmegaPeripherals;

#[cfg(feature = "stm32f1")]
use stm32f1::stm32f103::Peripherals as CortexPeripherals;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(feature = "atmega328p")]
#[no_mangle]
pub extern "C" fn main() -> ! {
    // Initialisation pour ATmega328P
    let peripherals = unsafe { AtmegaPeripherals::take().unwrap() };
    
    // Configuration LED sur pin 13 (Arduino built-in LED)
    let pin = AtmegaPin::new(&mut peripherals.PORTB as *mut _, 5);
    let mut led = Led::new(pin).unwrap();
    
    // Configuration USART
    let usart = AtmegaUsart::new(&mut peripherals.USART0 as *mut _);
    let mut serial = Serial::new(usart, 9600).unwrap();

    // Exemple d'utilisation
    loop {
        led.toggle().unwrap();
        serial.send_string("Hello from ATmega328P!\r\n").unwrap();
        
        // Attente simple
        for _ in 0..100000 {
            core::hint::spin_loop();
        }
    }
}

#[cfg(feature = "stm32f1")]
#[cortex_m_rt::entry]
fn main() -> ! {
    // Initialisation pour Cortex-M3
    let peripherals = unsafe { CortexPeripherals::take().unwrap() };
    
    // Configuration LED sur pin PA5 (souvent utilisée comme LED sur STM32)
    let pin = CortexPin::new(&mut peripherals.GPIOA as *mut _, 5);
    let mut led = Led::new(pin).unwrap();
    
    // Configuration USART
    let usart = CortexUsart::new(&mut peripherals.USART1 as *mut _);
    let mut serial = Serial::new(usart, 115200).unwrap();

    // Exemple d'utilisation
    loop {
        led.toggle().unwrap();
        serial.send_string("Hello from Cortex-M3!\r\n").unwrap();
        
        // Attente simple
        for _ in 0..1000000 {
            core::hint::spin_loop();
        }
    }
}
