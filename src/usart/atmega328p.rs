use super::USART;

const UBRR0H: *mut u8 = 0xC5 as *mut u8;
const UBRR0L: *mut u8 = 0xC4 as *mut u8;
const UCSR0A: *mut u8 = 0xC0 as *mut u8;
const UCSR0B: *mut u8 = 0xC1 as *mut u8;
const UCSR0C: *mut u8 = 0xC2 as *mut u8;
const UDR0: *mut u8 = 0xC6 as *mut u8;

pub struct Atmega328p;

impl USART for Atmega328p {
    fn init(baud_rate: u32) {
        let ubrr_value = (16_000_000 / (16 * baud_rate) - 1) as u16;
        unsafe {
            *UBRR0H = (ubrr_value >> 8) as u8;
            *UBRR0L = ubrr_value as u8;

            *UCSR0B = 1 << 3 | 1 << 4; // Enable TX (bit 3) and RX (bit 4)
            *UCSR0C = 1 << 1 | 1 << 2; // Set frame format: 8 data bits, 1 stop bit
        }
    }

    fn write(data: u8) {
        unsafe {
            while *UCSR0A & (1 << 5) == 0 {} // Wait for transmit buffer to be empty
            *UDR0 = data; // Load data into the buffer
        }
    }

    fn read() -> u8 {
        unsafe {
            while *UCSR0A & (1 << 7) == 0 {} // Wait for data to be received
            *UDR0 // Return received data
        }
    }
}

