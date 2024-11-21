pub mod atmega328p;
pub mod cortex_m3;

pub trait USART {
    fn init(baud_rate: u32);
    fn write(data: u8);
    fn read() -> u8;
}

// NE fonctionne pas, préciser votre features à chaque fois pour que ca compile
#[cfg(feature = "atmega328p")]
pub type ActiveUSART = atmega328p::Atmega328p;

#[cfg(feature = "cortex_m3")]
pub type ActiveUSART = cortex_m3::CortexM3;

pub fn init(baud_rate: u32) {
    ActiveUSART::init(baud_rate);
}

pub fn write(data: u8) {
    ActiveUSART::write(data);
}

pub fn read() -> u8 {
    ActiveUSART::read()
}

