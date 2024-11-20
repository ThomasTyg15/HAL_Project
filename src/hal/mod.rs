mod target;
pub use target::Target;

pub trait Peripheral {
    fn new() -> Self;
}
