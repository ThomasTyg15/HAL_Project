use crate::gpio::{GpioPin, Direction};
use crate::usart::UsartInterface;
use core::fmt::Write;

/// Driver pour LED avec contrôle basique
pub struct Led<T: GpioPin> {
    pin: T,
}

impl<T: GpioPin> Led<T> {
    pub fn new(mut pin: T) -> Result<Self, T::Error> {
        pin.set_direction(Direction::Output)?;
        Ok(Self { pin })
    }

    pub fn on(&mut self) -> Result<(), T::Error> {
        self.pin.set_high()
    }

    pub fn off(&mut self) -> Result<(), T::Error> {
        self.pin.set_low()
    }

    pub fn toggle(&mut self) -> Result<(), T::Error> {
        self.pin.toggle()
    }
}
