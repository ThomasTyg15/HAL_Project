use super::{Direction, GPIO};

const DDRB: *mut u8 = 0x24 as *mut u8;
const PORTB: *mut u8 = 0x25 as *mut u8;
const PINB: *mut u8 = 0x23 as *mut u8;

pub struct Atmega328p;

impl GPIO for Atmega328p {
    fn configure_pin(pin: u8, direction: Direction) {
        unsafe {
            match direction {
                Direction::Input => {
                    // Clear bit in DDRB to configure as input
                    *DDRB &= !(1 << pin);
                }
                Direction::Output => {
                    // Set bit in DDRB to configure as output
                    *DDRB |= 1 << pin;
                }
            }
        }
    }

    fn read_pin(pin: u8) -> bool {
        unsafe {
            // Read the state of the pin from PINB
            (*PINB & (1 << pin)) != 0
        }
    }

    fn write_pin(pin: u8, value: bool) {
        unsafe {
            if value {
                // Set bit in PORTB to write HIGH
                *PORTB |= 1 << pin;
            } else {
                // Clear bit in PORTB to write LOW
                *PORTB &= !(1 << pin);
            }
        }
    }
}

