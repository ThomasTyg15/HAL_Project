#[cfg(test)]
mod tests {
    use super::*;

    // Simuler la configuration d'une broche GPIO
    #[test]
    fn test_gpio_configuration() {
        // Simule la configuration de la broche 2 comme sortie
        configure_pin(2, Direction::Output);
        // Simule la lecture de la broche 2 après l'avoir configurée
        let state = read_pin(2);
        assert_eq!(state, false); // S'assurer que l'état est à LOW (si non initialisé à HIGH)
    }

    // Simuler l'écriture sur une broche GPIO
    #[test]
    fn test_gpio_write() {
        // Configurer la broche 2 comme sortie
        configure_pin(2, Direction::Output);
        // Simuler l'écriture HIGH sur la broche 2
        write_pin(2, true);
        let state = read_pin(2); // Lire l'état de la broche
        assert_eq!(state, true); // S'assurer que l'état est HIGH
    }
}
