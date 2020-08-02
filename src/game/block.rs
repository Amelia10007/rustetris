use super::Cell;
use crate::data_type::RowMajorTable;
use crate::geometry::*;

/// ブロックを表す．
/// ブロックは空でないセルの集合．
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    cells: RowMajorTable<Cell>,
    center: Position,
}

impl Block {
    pub fn new(cells: RowMajorTable<Cell>) -> Block {
        unimplemented!()
    }

    pub fn turn_clockwise(&self) -> Block {
        unimplemented!()
    }

    pub fn turn_unticlockwise(&self) -> Block {
        unimplemented!()
    }
}
