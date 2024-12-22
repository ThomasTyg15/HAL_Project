#![no_std]

use super::I2C;

const TWBR: *mut u8 = 0xB8 as *mut u8;
const TWSR: *mut u8 = 0xB9 as *mut u8;
const TWCR: *mut u8 = 0xBC as *mut u8;
const TWDR: *mut u8 = 0xBB as *mut u8;
const TWAR: *mut u8 = 0xBA as *mut u8;

pub struct Atmega328p;

impl I2C for Atmega328p {
    fn init(clock_speed: u32) {
        let prescaler = 1; // Prescaler par défaut (1x)
        let twbr_value = ((16_000_000 / clock_speed) - 16) / (2 * prescaler);
        unsafe {
            *TWBR = twbr_value as u8;
            *TWSR &= 0b11111100; // Clear prescaler bits
        }
    }

    fn write(address: u8, data: &[u8]) {
        unsafe {
            // Envoi du start condition
            *TWCR = 1 << 7 | 1 << 5 | 1 << 2;
            while *TWCR & (1 << 7) == 0 {}

            // Envoi de l'adresse
            *TWDR = (address << 1) & 0xFE; // Adresse + bit d'écriture
            *TWCR = 1 << 7 | 1 << 2;
            while *TWCR & (1 << 7) == 0 {}

            // Envoi des données
            for &byte in data {
                *TWDR = byte;
                *TWCR = 1 << 7 | 1 << 2;
                while *TWCR & (1 << 7) == 0 {}
            }

            // Envoi du stop condition
            *TWCR = 1 << 7 | 1 << 4 | 1 << 2;
        }
    }

    fn read(address: u8, buffer: &mut [u8]) {
        unsafe {
            // Envoi du start condition
            *TWCR = 1 << 7 | 1 << 5 | 1 << 2;
            while *TWCR & (1 << 7) == 0 {}

            // Envoi de l'adresse
            *TWDR = (address << 1) | 1; // Adresse + bit de lecture
            *TWCR = 1 << 7 | 1 << 2;
            while *TWCR & (1 << 7) == 0 {}

            // Réception des données
            for byte in buffer.iter_mut() {
                *TWCR = 1 << 7 | 1 << 2;
                while *TWCR & (1 << 7) == 0 {}
                *byte = *TWDR;
            }

            // Envoi du stop condition
            *TWCR = 1 << 7 | 1 << 4 | 1 << 2;
        }
    }
}
