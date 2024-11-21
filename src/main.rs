#![no_std]
#![no_main]

use cortex_m_rt::entry;
use core::arch::asm;

use my_hal_project::gpio::{configure_pin, write_pin, read_pin, Direction};
use my_hal_project::usart::{init as usart_init, read as usart_read}; // `usart_write` retiré car inutilisé
use my_hal_project::spi::{init as spi_init, transfer as spi_transfer};

#[entry]
fn main() -> ! {
    // Exemple GPIO
    configure_pin(2, Direction::Output);
    write_pin(2, true);
    let _gpio_state = read_pin(2); // Préfixé d'un _ pour éviter les warnings

    // Exemple USART
    usart_init(9600);
    let _received = usart_read(); // Préfixé d'un _ pour éviter les warnings

    // Exemple SPI
    spi_init();
    let _spi_response = spi_transfer(0x42); // Préfixé d'un _ pour éviter les warnings

    // Boucle infinie pour maintenir le programme actif
    loop {
        unsafe {
            asm!("nop");
        }
    }
}

