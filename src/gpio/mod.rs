pub mod atmega328p;
pub mod cortex_m3;

pub enum Direction {
    Input,
    Output,
}

pub trait GPIO {
    fn configure_pin(pin: u8, direction: Direction);
    fn read_pin(pin: u8) -> bool;
    fn write_pin(pin: u8, value: bool);
}

// NE fonctionne pas, préciser votre features à chaque fois pour que ca compile
#[cfg(feature = "atmega328p")]
pub type ActiveGPIO = atmega328p::Atmega328p;

#[cfg(feature = "cortex_m3")]
pub type ActiveGPIO = cortex_m3::CortexM3;

pub fn configure_pin(pin: u8, direction: Direction) {
    ActiveGPIO::configure_pin(pin, direction);
}

pub fn read_pin(pin: u8) -> bool {
    ActiveGPIO::read_pin(pin)
}

pub fn write_pin(pin: u8, value: bool) {
    ActiveGPIO::write_pin(pin, value);
}

