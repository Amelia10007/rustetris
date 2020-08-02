use super::Cell;
use crate::data_type::{RowMajorTable, Table, TableIndex, TableMut};
use crate::geometry::*;

/// ブロックを表す．
/// ブロックは空でないセルの集合．
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    cells: RowMajorTable<Cell>,
}

impl Block {
    pub fn new(cells: RowMajorTable<Cell>) -> Block {
        Self { cells }
    }

    /// 指定した位置のセルへの参照を返す．
    /// # Returns
    /// 1. 指定した位置にセルが存在する場合は`Some(cell)`を返す．
    /// 1. 指定した位置にセルが存在しない場合は`None`を返す．
    pub fn get(&self, p: Position) -> Option<&Cell> {
        let x = p.x().as_positive_index()?;
        let y = p.y().as_positive_index()?;
        self.cells.get(TableIndex::new(x, y))
    }

    /// 指定した位置のセルへの可変参照を返す．
    /// # Returns
    /// 1. 指定した位置にセルが存在する場合は`Some(cell)`を返す．
    /// 1. 指定した位置にセルが存在しない場合は`None`を返す．
    pub fn get_mut(&mut self, p: Position) -> Option<&mut Cell> {
        let x = p.x().as_positive_index()?;
        let y = p.y().as_positive_index()?;
        self.cells.get_mut(TableIndex::new(x, y))
    }

    /// このブロックを時計回りに90度だけ回転したものを返す．
    pub fn turn_clockwise(&self) -> Block {
        let mut table = RowMajorTable::from_fill(Cell::Empty, self.cells.size().swap());
        for (y, row) in self.cells.iter_row().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                let trans_y = x;
                let trans_x = self.cells.height() - y - 1;
                table[TableIndex::new(trans_x, trans_y)] = cell;
            }
        }
        Self { cells: table }
    }

    /// このブロックを反時計回りに90度だけ回転したものを返す．
    pub fn turn_unticlockwise(&self) -> Block {
        let mut table = RowMajorTable::from_fill(Cell::Empty, self.cells.size().swap());
        for (y, row) in self.cells.iter_row().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                let trans_y = self.cells.width() - x - 1;
                let trans_x = y;
                table[TableIndex::new(trans_x, trans_y)] = cell;
            }
        }
        Self { cells: table }
    }
}

/// フィールド内でエージェントの操作に応じて移動可能なブロックを表す．
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledBlock {
    /// ブロックの形状．
    block: Block,
    /// フィールドにおける，ブロックの左上セルの座標．
    left_top: Position,
}

impl ControlledBlock {
    pub const fn new(block: Block, left_top: Position) -> ControlledBlock {
        Self { block, left_top }
    }

    pub fn iter_position_and_cell(&self) -> impl Iterator<Item = (Position, &'_ Cell)> + '_ {
        self.block
            .cells
            .iter_row()
            .enumerate()
            .flat_map(move |(y, row)| {
                row.iter().enumerate().map(move |(x, cell)| {
                    let p = self.left_top + Movement(right(x as i8), below(y as i8));
                    (p, cell)
                })
            })
    }

    pub fn turn_clockwise(&self) -> ControlledBlock {
        unimplemented!()
    }

    pub fn turn_unticlockwise(&self) -> ControlledBlock {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests_block {
    use super::*;
    use Cell::*;

    fn verify_unticlockwise(block: Block) {
        assert_eq!(
            block.turn_clockwise(),
            block
                .turn_unticlockwise()
                .turn_unticlockwise()
                .turn_unticlockwise()
        );
        assert_eq!(
            block.turn_clockwise().turn_clockwise(),
            block.turn_unticlockwise().turn_unticlockwise()
        );
        assert_eq!(
            block.turn_clockwise().turn_clockwise().turn_clockwise(),
            block.turn_unticlockwise()
        );
    }

    #[test]
    fn test_turn_clockwise_single() {
        let single_block = Block::new(RowMajorTable::from_lines(vec![vec![Normal]]));
        let turned_block = single_block.turn_clockwise();
        assert_eq!(single_block, turned_block);
    }

    #[test]
    fn test_turn_clockwise_double() {
        // 横方向に，左から「Normal, Bomb」と並んでいるブロック
        let double_block = Block::new(RowMajorTable::from_lines(vec![vec![Normal, Bomb]]));
        // 回転
        let turn1 = double_block.turn_clockwise();
        assert_eq!(Some(&Normal), turn1.get(Position::origin()));
        assert_eq!(Some(&Bomb), turn1.get(Position::origin() + below(1)));

        let turn2 = turn1.turn_clockwise();
        assert_eq!(Some(&Bomb), turn2.get(Position::origin()));
        assert_eq!(Some(&Normal), turn2.get(Position::origin() + right(1)));

        let turn3 = turn2.turn_clockwise();
        assert_eq!(Some(&Bomb), turn3.get(Position::origin()));
        assert_eq!(Some(&Normal), turn3.get(Position::origin() + below(1)));

        assert_eq!(double_block, turn3.turn_clockwise());
    }

    #[test]
    fn test_turn_clockwise_square() {
        let square_block = Block::new(RowMajorTable::from_lines(vec![
            vec![Normal, Bomb],
            vec![BigBombUpperLeft, BigBombPart],
        ]));
        let turned_block = square_block.turn_clockwise();
        assert_eq!(BigBombUpperLeft, turned_block.cells[TableIndex::new(0, 0)]);
        assert_eq!(Normal, turned_block.cells[TableIndex::new(1, 0)]);
        assert_eq!(BigBombPart, turned_block.cells[TableIndex::new(0, 1)]);
        assert_eq!(Bomb, turned_block.cells[TableIndex::new(1, 1)]);
    }

    #[test]
    fn test_turn_clockwise_bar() {
        // 横方向に一列に並んだセルからなるブロック
        let bar_block = Block::new(RowMajorTable::from_lines(vec![vec![
            Normal,
            Bomb,
            BigBombUpperLeft,
            BigBombPart,
        ]]));
        let turn = bar_block.turn_clockwise();
        assert_eq!(Some(&Normal), turn.get(Position::origin()));
        assert_eq!(Some(&Bomb), turn.get(Position::origin() + below(1)));
        assert_eq!(
            Some(&BigBombUpperLeft),
            turn.get(Position::origin() + below(2))
        );
        assert_eq!(Some(&BigBombPart), turn.get(Position::origin() + below(3)));
    }

    #[test]
    fn test_turn_unticlockwise_single() {
        let single_block = Block::new(RowMajorTable::from_lines(vec![vec![Normal]]));
        verify_unticlockwise(single_block);
    }

    #[test]
    fn test_turn_unticlockwise_double() {
        // 横方向に，左から「Normal, Bomb」と並んでいるブロック
        let double_block = Block::new(RowMajorTable::from_lines(vec![vec![Normal, Bomb]]));
        verify_unticlockwise(double_block);
    }

    #[test]
    fn test_turn_unticlockwise_square() {
        let square_block = Block::new(RowMajorTable::from_lines(vec![
            vec![Normal, Bomb],
            vec![BigBombUpperLeft, BigBombPart],
        ]));
        verify_unticlockwise(square_block);
    }

    #[test]
    fn test_turn_unticlockwise_bar() {
        // 横方向に一列に並んだセルからなるブロック
        let bar_block = Block::new(RowMajorTable::from_lines(vec![vec![
            Normal,
            Bomb,
            BigBombUpperLeft,
            BigBombPart,
        ]]));
        verify_unticlockwise(bar_block);
    }
}

#[cfg(test)]
mod tests_controlled_block {
    use super::*;
    use Cell::*;

    #[test]
    fn test_iter_position_and_cell() {
        let controlled_block = {
            let block = Block::new(RowMajorTable::from_lines(vec![
                vec![Normal, Bomb],
                vec![BigBombUpperLeft, BigBombPart],
            ]));
            let left_top_position = Position(
                PositionX::origin() + right(1),
                PositionY::origin() + below(2),
            );
            ControlledBlock::new(block, left_top_position)
        };

        let mut iter = controlled_block.iter_position_and_cell();

        let (p, c) = iter.next().unwrap();
        assert_eq!(Position(PositionX::right(1), PositionY::below(2)), p);
        assert_eq!(&Cell::Normal, c);
        let (p, c) = iter.next().unwrap();
        assert_eq!(Position(PositionX::right(2), PositionY::below(2)), p);
        assert_eq!(&Cell::Bomb, c);
        let (p, c) = iter.next().unwrap();
        assert_eq!(Position(PositionX::right(1), PositionY::below(3)), p);
        assert_eq!(&Cell::BigBombUpperLeft, c);
        let (p, c) = iter.next().unwrap();
        assert_eq!(Position(PositionX::right(2), PositionY::below(3)), p);
        assert_eq!(&Cell::BigBombPart, c);

        assert!(iter.next().is_none());
    }
}
