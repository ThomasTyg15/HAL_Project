pub struct Hal {
    target: Target,
    #[cfg(feature = "atmega328p")]
    atmega: Option<atmega328p::Peripherals>,
    #[cfg(feature = "stm32f1")]
    cortex: Option<stm32f1::Peripherals>,
}

impl Hal {
    #[cfg(feature = "atmega328p")]
    pub fn new_atmega328p() -> Self {
        Self {
            target: Target::Atmega328p,
            atmega: Some(unsafe { atmega328p::Peripherals::take().unwrap() }),
            #[cfg(feature = "stm32f1")]
            cortex: None,
        }
    }

    #[cfg(feature = "stm32f1")]
    pub fn new_cortex_m3() -> Self {
        Self {
            target: Target::CortexM3,
            #[cfg(feature = "atmega328p")]
            atmega: None,
            cortex: Some(stm32f1::Peripherals::take()),
        }
    }

    pub fn create_usart(&mut self) -> Box<dyn UsartInterface<Error = Box<dyn std::error::Error>>> {
        match self.target {
            #[cfg(feature = "atmega328p")]
            Target::Atmega328p => Box::new(atmega328p::Usart::new(
                self.atmega.as_mut().unwrap().USART0
            )),
            #[cfg(feature = "stm32f1")]
            Target::CortexM3 => Box::new(cortex_m3::Usart::new(
                self.cortex.as_mut().unwrap().USART1
            )),
            _ => panic!("Target not supported"),
        }
    }
}
