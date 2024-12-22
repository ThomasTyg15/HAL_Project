#![no_std]

use super::I2C;

const I2C_CR1: *mut u32 = 0x40005400 as *mut u32;
const I2C_CR2: *mut u32 = 0x40005404 as *mut u32;
const I2C_DR: *mut u32 = 0x40005410 as *mut u32;
const I2C_SR1: *mut u32 = 0x40005414 as *mut u32;
const I2C_SR2: *mut u32 = 0x40005418 as *mut u32;

pub struct CortexM3;

impl I2C for CortexM3 {
    fn init(clock_speed: u32) {
        unsafe {
            // Configurer le périphérique I2C pour le clock_speed spécifié
            *I2C_CR2 = clock_speed / 1_000_000; // Clock speed en MHz
            *I2C_CR1 |= 1; // Activation du module I2C
        }
    }

    fn write(address: u8, data: &[u8]) {
        unsafe {
            // Start condition
            *I2C_CR1 |= 1 << 8;
            while *I2C_SR1 & (1 << 0) == 0 {}

            // Adresse
            *I2C_DR = (address << 1) as u32;
            while *I2C_SR1 & (1 << 1) == 0 {}
            *I2C_SR2;

            // Données
            for &byte in data {
                *I2C_DR = byte as u32;
                while *I2C_SR1 & (1 << 7) == 0 {}
            }

            // Stop condition
            *I2C_CR1 |= 1 << 9;
        }
    }

    fn read(address: u8, buffer: &mut [u8]) {
        unsafe {
            // Start condition
            *I2C_CR1 |= 1 << 8;
            while *I2C_SR1 & (1 << 0) == 0 {}

            // Adresse
            *I2C_DR = ((address << 1) | 1) as u32;
            while *I2C_SR1 & (1 << 1) == 0 {}
            *I2C_SR2;

            // Réception des données
            for byte in buffer.iter_mut() {
                while *I2C_SR1 & (1 << 6) == 0 {}
                *byte = *I2C_DR as u8;
            }

            // Stop condition
            *I2C_CR1 |= 1 << 9;
        }
    }
}
