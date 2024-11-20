use avr_device::atmega328p::{USART0};
use super::traits::UsartInterface;

pub struct AtmegaUsart {
    usart: *mut USART0,
}

#[derive(Debug)]
pub struct AtmegaUsartError;

impl AtmegaUsart {
    pub fn new(usart: *mut USART0) -> Self {
        Self { usart }
    }

    fn calculate_ubrr(baud_rate: u32) -> u16 {
        let clock_rate: u32 = 16_000_000;
        ((clock_rate / 16) / baud_rate - 1) as u16
    }
}

impl UsartInterface for AtmegaUsart {
    type Error = AtmegaUsartError;

    fn init(&mut self, baud_rate: u32) -> Result<(), Self::Error> {
        let ubrr = Self::calculate_ubrr(baud_rate);
        unsafe {
            (*self.usart).ubrr0h.write(|w| w.bits((ubrr >> 8) as u8));
            (*self.usart).ubrr0l.write(|w| w.bits(ubrr as u8));
            (*self.usart).ucsr0b.write(|w| w
                .rxen0().set_bit()
                .txen0().set_bit()
            );
            (*self.usart).ucsr0c.write(|w| w
                .ucsz00().set_bit()
                .ucsz01().set_bit()
            );
        }
        Ok(())
    }

    fn send_byte(&mut self, byte: u8) -> Result<(), Self::Error> {
        while !self.is_tx_ready()? {}
        unsafe {
            (*self.usart).udr0.write(|w| w.bits(byte));
        }
        Ok(())
    }

    fn receive_byte(&mut self) -> Result<u8, Self::Error> {
        while !self.is_rx_ready()? {}
        unsafe {
            Ok((*self.usart).udr0.read().bits())
        }
    }

    fn is_rx_ready(&self) -> Result<bool, Self::Error> {
        unsafe {
            Ok((*self.usart).ucsr0a.read().rxc0().bit_is_set())
        }
    }

    fn is_tx_ready(&self) -> Result<bool, Self::Error> {
        unsafe {
            Ok((*self.usart).ucsr0a.read().udre0().bit_is_set())
        }
    }

    fn send_bytes(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
        for &byte in bytes {
            self.send_byte(byte)?;
        }
        Ok(())
    }

    fn read_bytes(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> {
        for (i, byte) in buffer.iter_mut().enumerate() {
            *byte = self.receive_byte()?;
            if *byte == b'\n' {
                return Ok(i + 1);
            }
        }
        Ok(buffer.len())
    }
}
