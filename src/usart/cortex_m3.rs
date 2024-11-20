use stm32f1::stm32f103::USART1;
use super::traits::UsartInterface;

pub struct CortexUsart {
    usart: *mut USART1,
}

#[derive(Debug)]
pub struct CortexUsartError;

impl CortexUsart {
    pub fn new(usart: *mut USART1) -> Self {
        Self { usart }
    }

    fn calculate_brr(baud_rate: u32) -> u16 {
        let clock_rate: u32 = 72_000_000;
        ((clock_rate + (baud_rate / 2)) / baud_rate) as u16
    }
}

impl UsartInterface for CortexUsart {
    type Error = CortexUsartError;

    fn init(&mut self, baud_rate: u32) -> Result<(), Self::Error> {
        let brr = Self::calculate_brr(baud_rate);
        unsafe {
            (*self.usart).brr.write(|w| w.bits(brr as u32));
            (*self.usart).cr1.modify(|_, w| w
                .ue().set_bit()
                .te().set_bit()
                .re().set_bit()
            );
        }
        Ok(())
    }

    fn send_byte(&mut self, byte: u8) -> Result<(), Self::Error> {
        while !self.is_tx_ready()? {}
        unsafe {
            (*self.usart).dr.write(|w| w.bits(byte as u32));
        }
        Ok(())
    }

    fn receive_byte(&mut self) -> Result<u8, Self::Error> {
        while !self.is_rx_ready()? {}
        unsafe {
            Ok((*self.usart).dr.read().bits() as u8)
        }
    }

    fn is_rx_ready(&self) -> Result<bool, Self::Error> {
        unsafe {
            Ok((*self.usart).sr.read().rxne().bit_is_set())
        }
    }

    fn is_tx_ready(&self) -> Result<bool, Self::Error> {
        unsafe {
            Ok((*self.usart).sr.read().txe().bit_is_set())
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
