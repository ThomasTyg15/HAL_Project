use super::SPI;

const SPCR: *mut u8 = 0x4C as *mut u8;
const SPSR: *mut u8 = 0x4D as *mut u8;
const SPDR: *mut u8 = 0x4E as *mut u8;

pub struct Atmega328p;

impl SPI for Atmega328p {
    fn init() {
        unsafe {
            // Configurer le SPI en mode maître, avec une fréquence d'horloge /16
            *SPCR = (1 << 6) | (1 << 4) | (1 << 1); // SPI Enable, Master Mode, Fclk/16
            *SPSR = 0; // Désactiver Double SPI Speed
        }
    }

    fn write(data: u8) {
        unsafe {
            *SPDR = data; // Charger les données à transmettre dans le registre SPDR
            while *SPSR & (1 << 7) == 0 {} // Attendre la fin de la transmission (bit SPIF)
        }
    }

    fn read() -> u8 {
        unsafe {
            while *SPSR & (1 << 7) == 0 {} // Attendre que des données soient reçues
            *SPDR // Retourner les données reçues
        }
    }
}

