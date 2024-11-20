use avr_device::atmega328p::{PORT};
use super::traits::{GpioPin, Direction, PinMode};


use panic_halt as _;  // Used to handle panic

// Pointers to Atmega328p registers
const DDRB: *mut u8 = 0x24 as *mut u8;  // Data Direction Register B (PORTB)
const PORTB: *mut u8 = 0x25 as *mut u8; // Output Register B
const PINB: *mut u8 = 0x23 as *mut u8;  // Input Pins Address for PORTB
const DDRD: *mut u8 = 0x2A as *mut u8;  // Data Direction Register D (PORTD)
const PORTD: *mut u8 = 0x2B as *mut u8; // Output Register D
const PIND: *mut u8 = 0x29 as *mut u8;  // Input Pins Address for PORTD

// Select pin number (0 to 13)
const PIN_NUMBER: u8 = 13; // Change the pin number here

// Set mode (true for output, false for input)
const IS_OUTPUT: bool = true; // Change to `false` for input configuration

// Set desired output state (true for HIGH, false for LOW) if in output mode
const OUTPUT_STATE: bool = true; // Change to `false` for LOW

#[arduino_hal::entry]
fn main() -> ! {
    unsafe {
        // Configure pin as input or output
        if IS_OUTPUT {
            configure_pin_as_output(PIN_NUMBER);
            write_pin(PIN_NUMBER, OUTPUT_STATE);
        } else {
            configure_pin_as_input(PIN_NUMBER);
            let input_state = read_pin(PIN_NUMBER);
            // Process input_state if needed
        }
    }

    loop {}  // Infinite loop to maintain state
}

/// Configure a pin as output
unsafe fn configure_pin_as_output(pin: u8) {
    match pin {
        0 => *DDRD |= 1 << 0,
        1 => *DDRD |= 1 << 1,
        2 => *DDRD |= 1 << 2,
        3 => *DDRD |= 1 << 3,
        4 => *DDRD |= 1 << 4,
        5 => *DDRD |= 1 << 5,
        6 => *DDRD |= 1 << 6,
        7 => *DDRD |= 1 << 7,
        8  => *DDRB |= 1 << 0,
        9  => *DDRB |= 1 << 1,
        10 => *DDRB |= 1 << 2,
        11 => *DDRB |= 1 << 3,
        12 => *DDRB |= 1 << 4,
        13 => *DDRB |= 1 << 5,
        _ => panic!("Unsupported pin"),
    }
}

/// Configure a pin as input
unsafe fn configure_pin_as_input(pin: u8) {
    match pin {
        0 => *DDRD &= !(1 << 0),
        1 => *DDRD &= !(1 << 1),
        2 => *DDRD &= !(1 << 2),
        3 => *DDRD &= !(1 << 3),
        4 => *DDRD &= !(1 << 4),
        5 => *DDRD &= !(1 << 5),
        6 => *DDRD &= !(1 << 6),
        7 => *DDRD &= !(1 << 7),
        8  => *DDRB &= !(1 << 0),
        9  => *DDRB &= !(1 << 1),
        10 => *DDRB &= !(1 << 2),
        11 => *DDRB &= !(1 << 3),
        12 => *DDRB &= !(1 << 4),
        13 => *DDRB &= !(1 << 5),
        _ => panic!("Unsupported pin"),
    }
}

/// Write a value (HIGH or LOW) to a pin configured as output
unsafe fn write_pin(pin: u8, state: bool) {
    if state {
        match pin {
            0 => *PORTD |= 1 << 0,
            1 => *PORTD |= 1 << 1,
            2 => *PORTD |= 1 << 2,
            3 => *PORTD |= 1 << 3,
            4 => *PORTD |= 1 << 4,
            5 => *PORTD |= 1 << 5,
            6 => *PORTD |= 1 << 6,
            7 => *PORTD |= 1 << 7,
            8  => *PORTB |= 1 << 0,
            9  => *PORTB |= 1 << 1,
            10 => *PORTB |= 1 << 2,
            11 => *PORTB |= 1 << 3,
            12 => *PORTB |= 1 << 4,
            13 => *PORTB |= 1 << 5,
            _ => panic!("Unsupported pin"),
        }
    } else {
        match pin {
            0 => *PORTD &= !(1 << 0),
            1 => *PORTD &= !(1 << 1),
            2 => *PORTD &= !(1 << 2),
            3 => *PORTD &= !(1 << 3),
            4 => *PORTD &= !(1 << 4),
            5 => *PORTD &= !(1 << 5),
            6 => *PORTD &= !(1 << 6),
            7 => *PORTD &= !(1 << 7),
            8  => *PORTB &= !(1 << 0),
            9  => *PORTB &= !(1 << 1),
            10 => *PORTB &= !(1 << 2),
            11 => *PORTB &= !(1 << 3),
            12 => *PORTB &= !(1 << 4),
            13 => *PORTB &= !(1 << 5),
            _ => panic!("Unsupported pin"),
        }
    }
}

/// Read the state of a pin configured as input (returns true for HIGH, false for LOW)
unsafe fn read_pin(pin: u8) -> bool {
    match pin {
        0 => (*PIND & (1 << 0)) != 0,
        1 => (*PIND & (1 << 1)) != 0,
        2 => (*PIND & (1 << 2)) != 0,
        3 => (*PIND & (1 << 3)) != 0,
        4 => (*PIND & (1 << 4)) != 0,
        5 => (*PIND & (1 << 5)) != 0,
        6 => (*PIND & (1 << 6)) != 0,
        7 => (*PIND & (1 << 7)) != 0,
        8  => (*PINB & (1 << 0)) != 0,
        9  => (*PINB & (1 << 1)) != 0,
        10 => (*PINB & (1 << 2)) != 0,
        11 => (*PINB & (1 << 3)) != 0,
        12 => (*PINB & (1 << 4)) != 0,
        13 => (*PINB & (1 << 5)) != 0,
        _ => panic!("Unsupported pin"),
    }
}
