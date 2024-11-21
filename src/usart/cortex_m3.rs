use super::USART;

const USART2_BASE: usize = 0x40004400;
const USART2_SR: *mut u32 = (USART2_BASE + 0x00) as *mut u32;
const USART2_DR: *mut u32 = (USART2_BASE + 0x04) as *mut u32;
const USART2_BRR: *mut u32 = (USART2_BASE + 0x08) as *mut u32;
const USART2_CR1: *mut u32 = (USART2_BASE + 0x0C) as *mut u32;

pub struct CortexM3;

impl USART for CortexM3 {
    fn init(baud_rate: u32) {
        let baud_div = 16_000_000 / baud_rate; 
        unsafe {
            *USART2_BRR = baud_div; // Set baud rate
            *USART2_CR1 = 1 << 3 | 1 << 2 | 1 << 13; // Enable TX, RX, and USART
        }
    }

    fn write(data: u8) {
        unsafe {
            while *USART2_SR & (1 << 7) == 0 {} // Wait for TXE (Transmit Data Register Empty)
            *USART2_DR = data as u32; // Write data to DR
        }
    }

    fn read() -> u8 {
        unsafe {
            while *USART2_SR & (1 << 5) == 0 {} // Wait for RXNE (Read Data Register Not Empty)
            *USART2_DR as u8 // Read data from DR
        }
    }
}

