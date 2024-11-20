mod traits;
mod atmega328p;
mod cortex_m3;

pub use traits::UsartInterface;
pub use atmega328p::AtmegaUsart;
pub use cortex_m3::CortexUsart;
