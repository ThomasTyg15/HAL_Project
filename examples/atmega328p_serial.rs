#![no_std]
#![no_main]

use my_hal::{Hal, usart::UsartInterface};
use panic_halt as _;

#[no_mangle]
pub extern "C" fn main() {
    let mut hal = Hal::new_atmega328p();
    let mut serial = hal.create_usart();
    
    serial.init(9600).unwrap();
    
    loop {
        // Echo back received data
        if let Ok(byte) = serial.receive_byte() {
            let _ = serial.send_byte(byte);
        }
    }
}
