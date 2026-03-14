use crate::types::{Bar, Signal};

pub trait Strategy {
    fn name(&self) -> &str;
    fn generate_signal(&self, bars: &[Bar], current_index: usize) -> Signal;
}
