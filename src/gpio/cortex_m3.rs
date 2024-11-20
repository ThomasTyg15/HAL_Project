m32f103::GPIO;
use super::traits::{GpioPin, Direction, PinMode};

pub struct CortexPin {
    gpio: *mut GPIO,
    pin: u8,
}

impl CortexPin {
    pub fn new(gpio: *mut GPIO, pin: u8) -> Self {
        Self { gpio, pin }
    }
}

#[derive(Debug)]
pub struct CortexError;

impl GpioPin for CortexPin {
    type Error = CortexError;

    fn set_direction(&mut self, direction: Direction) -> Result<(), Self::Error> {
        unsafe {
            let offset = (self.pin as usize) * 4;
            match direction {
                Direction::Output => {
                    (*self.gpio).cr[self.pin as usize / 8]
                        .modify(|r, w| w.bits((r.bits() & !(0xF << offset)) | (0x3 << offset)));
                }
                Direction::Input => {
                    (*self.gpio).cr[self.pin as usize / 8]
                        .modify(|r, w| w.bits((r.bits() & !(0xF << offset)) | (0x4 << offset)));
                }
            }
        }
        Ok(())
    }

    fn set_mode(&mut self, mode: PinMode) -> Result<(), Self::Error> {
        unsafe {
            let offset = (self.pin as usize) * 4;
            match mode {
                PinMode::PullUp => {
                    (*self.gpio).odr.modify(|r, w| w.bits(r.bits() | (1 << self.pin)));
                }
                PinMode::PullDown => {
                    (*self.gpio).odr.modify(|r, w| w.bits(r.bits() & !(1 << self.pin)));
                }
                PinMode::Floating => {
                    (*self.gpio).cr[self.pin as usize / 8]
                        .modify(|r, w| w.bits((r.bits() & !(0xF << offset)) | (0x4 << offset)));
                }
            }
        }
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        unsafe {
            (*self.gpio).bsrr.write(|w| w.bits(1 << self.pin));
        }
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        unsafe {
            (*self.gpio).brr.write(|w| w.bits(1 << self.pin));
        }
        Ok(())
    }

    fn is_high(&self) -> Result<bool, Self::Error> {
        unsafe {
            Ok(((*self.gpio).idr.read().bits() & (1 << self.pin)) != 0)
        }
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        self.is_high().map(|v| !v)
    }

    fn toggle(&mut self) -> Result<(), Self::Error> {
        if self.is_high()? {
            self.set_low()
        } else {
            self.set_high()
        }
    }
}
