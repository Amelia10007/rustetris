use super::block_template::*;
use super::Cell;
use crate::geometry::*;

/// フィールド内でエージェントの操作に応じて移動可能なブロックを表す．
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledBlock {
    /// ブロック．
    block: Block,
    /// フィールドにおける，ブロックの左上セルの座標．
    left_top: Pos,
}

impl ControlledBlock {
    pub const fn new(block: Block, left_top: Pos) -> ControlledBlock {
        Self { block, left_top }
    }

    /// このブロックに含まれるセルおよびフィールドにおけるその位置をを列挙する．
    /// # Returns
    /// このブロックに含まれるセルと，フィールドにおけるその位置を`(pos, cell)`として列挙する`Iterator`．
    /// イテレータはまず最上段の左端のセルを返し，以降その右隣のセルを返し続ける．
    /// 最上段のセルをすべて列挙し終えたら，次はひとつ下の段にあるセルを左端から列挙する．
    /// イテレータは，最下段のセルまで以上の操作を繰り返し行う．
    pub fn iter_pos_and_occupied_cell(&self) -> impl Iterator<Item = (Pos, &'_ Cell)> + '_ {
        // ブロックは1つ以上のセルから構成されるので，ここのunwrap()は必ず成功する．
        let diff = self.left_top - self.block.iter_pos_and_cell().next().unwrap().0;
        self.block
            .iter_pos_and_cell()
            .map(move |(pos, cell)| (pos + diff, cell))
    }

    pub fn move_by(&self, movement: Movement) -> ControlledBlock {
        Self {
            block: self.block,
            left_top: self.left_top + movement,
        }
    }

    pub fn rotate_clockwise(&self) -> ControlledBlock {
        Self::new(self.block.rotate_clockwise(), self.left_top)
    }

    pub fn rotate_unticlockwise(&self) -> ControlledBlock {
        Self::new(self.block.rotate_unticlockwise(), self.left_top)
    }
}
