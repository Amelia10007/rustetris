use std::marker::PhantomData;

mod block_template;
mod cell;
mod field;
mod field_under_agent_control;
mod single_play;

pub use block_template::*;
pub use cell::Cell;
pub use field::Field;
pub use field_under_agent_control::FieldUnderAgentControl;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Progress {
    Progressive,
    Done,
}

pub trait Animation {
    type Interpolation;

    fn current(&self) -> Self::Interpolation;

    fn update(&mut self) -> Progress;

    fn wait_next_update(&mut self);
}
