use super::SPI;

const SPI1_BASE: usize = 0x40013000;
const SPI1_CR1: *mut u32 = (SPI1_BASE + 0x00) as *mut u32;
const SPI1_SR: *mut u32 = (SPI1_BASE + 0x08) as *mut u32;
const SPI1_DR: *mut u32 = (SPI1_BASE + 0x0C) as *mut u32;

pub struct CortexM3;

impl SPI for CortexM3 {
    fn init() {
        unsafe {
            // Configurer le SPI en mode maître avec un baudrate divisor de 16
            *SPI1_CR1 = (1 << 2) // Master Mode
                      | (1 << 6) // SPI Enable
                      | (0b011 << 3); // Baudrate Divisor Fclk/16
        }
    }

    fn write(data: u8) {
        unsafe {
            while *SPI1_SR & (1 << 1) == 0 {} // Attendre que le tampon TX soit vide (bit TXE)
            *SPI1_DR = data as u32; // Écrire dans le registre de données
        }
    }

    fn read() -> u8 {
        unsafe {
            while *SPI1_SR & (1 << 0) == 0 {} // Attendre que les données soient reçues (bit RXNE)
            *SPI1_DR as u8 // Lire les données reçues
        }
    }
}

