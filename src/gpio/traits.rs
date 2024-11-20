pub trait GpioPin {
    fn set_high(&mut self);
    fn set_low(&mut self);
    fn is_high(&self) -> bool;
    fn is_low(&self) -> bool;
    fn set_direction(&mut self, direction: Direction);
}

pub enum Direction {
    Input,
    Output,
}
