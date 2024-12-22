#![no_std]

pub mod atmega328p;
pub mod cortex_m3;

/// Interface I2C générique.
pub trait I2C {
    fn init(clock_speed: u32);
    fn write(address: u8, data: &[u8]);
    fn read(address: u8, buffer: &mut [u8]);
}
