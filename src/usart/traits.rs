pub trait UsartInterface {
    type Error;

    fn init(&mut self, baud_rate: u32) -> Result<(), Self::Error>;
    fn send_byte(&mut self, byte: u8) -> Result<(), Self::Error>;
    fn receive_byte(&mut self) -> Result<u8, Self::Error>;
    fn is_rx_ready(&self) -> Result<bool, Self::Error>;
    fn is_tx_ready(&self) -> Result<bool, Self::Error>;
    fn send_bytes(&mut self, bytes: &[u8]) -> Result<(), Self::Error>;
    fn read_bytes(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error>;
}
