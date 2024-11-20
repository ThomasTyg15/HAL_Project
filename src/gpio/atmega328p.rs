use avr_device::atmega328p::{PORT};
use super::traits::{GpioPin, Direction, PinMode};

pub struct AtmegaPin {
    port: *mut PORT,
    pin: u8,
}

impl AtmegaPin {
    pub fn new(port: *mut PORT, pin: u8) -> Self {
        Self { port, pin }
    }
}

#[derive(Debug)]
pub struct AtmegaError;

impl GpioPin for AtmegaPin {
    type Error = AtmegaError;

    fn set_direction(&mut self, direction: Direction) -> Result<(), Self::Error> {
        unsafe {
            match direction {
                Direction::Output => (*self.port).ddr.modify(|r, w| w.bits(r.bits() | (1 << self.pin))),
                Direction::Input => (*self.port).ddr.modify(|r, w| w.bits(r.bits() & !(1 << self.pin))),
            }
        }
        Ok(())
    }

    fn set_mode(&mut self, mode: PinMode) -> Result<(), Self::Error> {
        unsafe {
            match mode {
                PinMode::PullUp => {
                    (*self.port).port.modify(|r, w| w.bits(r.bits() | (1 << self.pin)));
                }
                PinMode::Floating => {
                    (*self.port).port.modify(|r, w| w.bits(r.bits() & !(1 << self.pin)));
                }
                PinMode::PullDown => return Err(AtmegaError), // ATmega328P doesn't support pull-down
            }
        }
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            (*self.port).port.modify(|r, w| w.bits(r.bits() | (1 << self.pin)));
        }
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe {
            (*self.port).port.modify(|r, w| w.bits(r.bits() & !(1 << self.pin)));
        }
        Ok(())
    }

    fn is_high(&self) -> Result<bool, Self::Error> {
        unsafe {
            Ok(((*self.port).pin.read().bits() & (1 << self.pin)) != 0)
        }
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        self.is_high().map(|v| !v)
    }

    fn toggle(&mut self) -> Result<(), Self::Error> {
        unsafe {
            (*self.port).pin.modify(|r, w| w.bits(r.bits() ^ (1 << self.pin)));
        }
        Ok(())
    }
}
