use super::{Direction, GPIO};

const GPIOA_MODER: *mut u32 = 0x48000000 as *mut u32;
const GPIOA_ODR: *mut u32 = 0x48000014 as *mut u32;
const GPIOA_IDR: *mut u32 = 0x48000010 as *mut u32;

pub struct CortexM3;

impl GPIO for CortexM3 {
    fn configure_pin(pin: u8, direction: Direction) {
        unsafe {
            match direction {
                Direction::Input => {
                    // Clear bits in MODER register to configure as input
                    *GPIOA_MODER &= !(0b11 << (pin * 2));
                }
                Direction::Output => {
                    // Set bits in MODER register to configure as output
                    *GPIOA_MODER &= !(0b11 << (pin * 2));
                    *GPIOA_MODER |= 0b01 << (pin * 2);
                }
            }
        }
    }

    fn read_pin(pin: u8) -> bool {
        unsafe {
            // Read the state of the pin from IDR
            (*GPIOA_IDR & (1 << pin)) != 0
        }
    }

    fn write_pin(pin: u8, value: bool) {
        unsafe {
            if value {
                // Set bit in ODR to write HIGH
                *GPIOA_ODR |= 1 << pin;
            } else {
                // Clear bit in ODR to write LOW
                *GPIOA_ODR &= !(1 << pin);
            }
        }
    }
}

