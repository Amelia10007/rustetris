use super::{Block, ControlledBlock, Field};
use std::collections::VecDeque;

pub struct AgentField {
    field: Field,
    current_block: ControlledBlock,
    next_blocks: VecDeque<Block>,
}

impl AgentField {
    pub fn new() -> AgentField {
        unimplemented!()
    }
}
