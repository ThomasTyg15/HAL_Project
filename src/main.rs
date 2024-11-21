use my_hal_project::gpio::{configure_pin, write_pin, read_pin, Direction};
use my_hal_project::usart::{init as usart_init, write as usart_write, read as usart_read};
use my_hal_project::spi::{init as spi_init, transfer as spi_transfer};


fn main() {
    // --- GPIO Example ---
    println!("--- GPIO Example ---");
    configure_pin(2, Direction::Output); // Configurer la broche 2 comme sortie
    write_pin(2, true); // Écrire HIGH sur la broche 2
    let gpio_state = read_pin(2); // Lire l'état de la broche 2
    println!("GPIO Pin 2 State: {}", gpio_state);

    // --- USART Example ---
    println!("--- USART Example ---");
    usart_init(9600); // Initialiser l'USART avec un baud rate de 9600
    usart_write(b'H'); // Envoyer 'H' via USART
    let received = usart_read(); // Lire un octet reçu via USART
    println!("USART Received: {}", received as char);

    // --- SPI Example ---
    println!("--- SPI Example ---");
    spi_init(); // Initialiser le SPI
    let spi_response = spi_transfer(0x42); // Transférer un octet via SPI et lire la réponse
    println!("SPI Response: 0x{:02X}", spi_response);
}

