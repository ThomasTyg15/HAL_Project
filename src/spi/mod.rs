pub mod atmega328p;
pub mod cortex_m3;

pub trait SPI {
    fn init();
    fn write(data: u8);
    fn read() -> u8;
    fn transfer(data: u8) -> u8 {
        Self::write(data);
        Self::read()
    }
}

// NE fonctionne pas, préciser votre features à chaque fois pour que ca compile
#[cfg(feature = "atmega328p")]
pub type ActiveSPI = atmega328p::Atmega328p;

#[cfg(feature = "cortex_m3")]
pub type ActiveSPI = cortex_m3::CortexM3;

pub fn init() {
    ActiveSPI::init();
}

pub fn write(data: u8) {
    ActiveSPI::write(data);
}

pub fn read() -> u8 {
    ActiveSPI::read()
}

pub fn transfer(data: u8) -> u8 {
    ActiveSPI::transfer(data)
}

