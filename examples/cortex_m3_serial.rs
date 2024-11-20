#![no_std]
#![no_main]

use cortex_m_rt::entry;
use my_hal::{Hal, usart::UsartInterface};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut hal = Hal::new_cortex_m3();
    let mut serial = hal.create_usart();
    
    serial.init(115200).unwrap();
    
    loop {
        // Echo back received data
        if let Ok(byte) = serial.receive_byte() {
            let _ = serial.send_byte(byte);
        }
    }
}
