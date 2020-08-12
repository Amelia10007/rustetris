use std::marker::PhantomData;

mod agent_field;
mod block;
mod block_template;
mod cell;
mod field;
mod single_play;

pub use agent_field::AgentField;
pub use block::ControlledBlock;
pub use block_template::*;
pub use cell::Cell;
pub use field::Field;

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
