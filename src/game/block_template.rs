use super::Cell;
use crate::geometry::*;
use std::ops::Index;

mod consts {
    /// ブロックの形状を定義するテーブルの一片の長さ．
    pub const BLOCK_TABLE_SIZE: usize = 5;
    /// ブロックの向きの数．
    pub const ROTATION_KIND: usize = 4;
}

use consts::*;

/// ブロック形状テンプレートに利用するタグ．
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellTag {
    /// このセルが空でないことを表す．
    /// また，0から始まる互いに異なるラベルが関連付けられる．
    Occupied(usize),
    /// このセルが空であることを表す．
    Empty,
}

/// ボムセルの位置を表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BombTag {
    /// ボムセルはない．すべての空でないセルは通常のセル．
    None,
    /// 空でないセルの中にひとつだけボムセルがあり，そのセルは指定したラベルと関連付けられている．
    Single(usize),
    /// すべての空でないセルがボムセルである．
    All,
}

/// ブロックの方向を表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Below,
    Right,
    Above,
}

impl Direction {
    fn rotate_clockwise(self) -> Direction {
        use Direction::*;

        match self {
            Left => Above,
            Above => Right,
            Right => Below,
            Below => Left,
        }
    }

    fn rotate_unticlockwise(self) -> Direction {
        use Direction::*;

        match self {
            Left => Below,
            Below => Right,
            Right => Above,
            Above => Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellCount {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl CellCount {
    pub fn as_integer(self) -> usize {
        match self {
            CellCount::One => 1,
            CellCount::Two => 2,
            CellCount::Three => 3,
            CellCount::Four => 4,
            CellCount::Five => 5,
        }
    }
}

/// ブロック生成ルールを表す．
pub trait BlockSelector {
    /// 空でないセルがいくつ含まれているブロックを生成するか返す．
    fn select_cell_count(&self) -> CellCount;

    /// 空でないセルを指定した数だけ含むブロックテンプレートが`n`個存在し，
    /// それぞれが`0,1,...,n-1`とラベル付けされているとき，
    /// 何番目のブロックテンプレートを利用するか返す．
    fn select_block_kind(&self, block_kind_num: usize) -> usize;

    /// ボムセルの数および位置を返す．
    fn select_bomb(&self, cell_count: CellCount) -> BombTag;

    /// ブロックを生成して返す．
    fn generate_block(&self) -> Block {
        let cell_count = self.select_cell_count();

        let collections = match cell_count {
            CellCount::One => &block_template::single::COLLECTIONS[..],
            CellCount::Two => &block_template::double::COLLECTIONS[..],
            CellCount::Three => &block_template::triple::COLLECTIONS[..],
            CellCount::Four => &block_template::quadruple::COLLECTIONS[..],
            CellCount::Five => &block_template::quintuple::COLLECTIONS[..],
        };
        let tables = &collections[self.select_block_kind(collections.len())];
        let bomb_tag = self.select_bomb(cell_count);

        Block::new(tables, Direction::Above, bomb_tag)
    }
}

/// ブロックテンプレートに利用される2次元テーブルデータ構造を定義する．
type Table<T> = [[T; BLOCK_TABLE_SIZE]; BLOCK_TABLE_SIZE];

/// ブロックの方向ごとにブロックの形状を定義する．
/// 内部は4要素の配列からなり，先頭から順に`Right,Below,Left,Above`の順に形状データが格納される．
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CellTagTableCollection([Table<CellTag>; ROTATION_KIND]);

impl Index<Direction> for CellTagTableCollection {
    type Output = Table<CellTag>;

    fn index(&self, d: Direction) -> &Self::Output {
        use Direction::*;

        let index = match d {
            Right => 0,
            Below => 1,
            Left => 2,
            Above => 3,
        };
        &self.0[index]
    }
}

/// ブロックを表す．
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    cells: Table<Cell>,
    tables: &'static CellTagTableCollection,
    direction: Direction,
    bomb_tag: BombTag,
}

impl Block {
    fn new(
        tables: &'static CellTagTableCollection,
        direction: Direction,
        bomb_tag: BombTag,
    ) -> Block {
        let cells = Self::generate_cells(tables, direction, bomb_tag);
        Self {
            cells,
            tables,
            direction,
            bomb_tag,
        }
    }

    pub fn rotate_clockwise(&self) -> Block {
        let direction = self.direction.rotate_clockwise();
        let cells = Self::generate_cells(self.tables, direction, self.bomb_tag);
        Block {
            cells,
            direction,
            ..*self
        }
    }

    pub fn rotate_unticlockwise(&self) -> Block {
        let direction = self.direction.rotate_unticlockwise();
        let cells = Self::generate_cells(self.tables, direction, self.bomb_tag);
        Block {
            cells,
            direction,
            ..*self
        }
    }

    fn generate_cells(
        tables: &'static CellTagTableCollection,
        direction: Direction,
        bomb_tag: BombTag,
    ) -> Table<Cell> {
        let mut cells = [[Cell::Empty; BLOCK_TABLE_SIZE]; BLOCK_TABLE_SIZE];
        for (source_row, target_row) in tables[direction].iter().zip(cells.iter_mut()) {
            for (&source, target) in source_row.iter().zip(target_row.iter_mut()) {
                *target = match source {
                    CellTag::Occupied(i) => match bomb_tag {
                        BombTag::All => Cell::Bomb,
                        BombTag::None => Cell::Normal,
                        BombTag::Single(j) => {
                            if i == j {
                                Cell::Bomb
                            } else {
                                Cell::Normal
                            }
                        }
                    },
                    CellTag::Empty => Cell::Empty,
                };
            }
        }
        cells
    }
}

/// ブロックの形状を定義するテンプレートを詰めたモジュール．
mod block_template {
    use super::CellTag::{Empty, Occupied};
    use super::CellTagTableCollection;

    /// 空でないセルひとつだけからなるブロックの形状を定義するモジュール．
    pub(super) mod single {
        use super::*;

        const SINGLE: CellTagTableCollection = CellTagTableCollection([
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(0), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(0), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(0), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(0), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
        ]);

        pub(in super::super) const COLLECTIONS: [CellTagTableCollection; 1] = [SINGLE];
    }

    /// 空でないセルふたつからなるブロックの形状を定義するモジュール．
    pub(super) mod double {
        use super::*;

        const DOUBLE: CellTagTableCollection = CellTagTableCollection([
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(0), Occupied(1), Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(0), Empty, Empty],
                [Empty, Empty, Occupied(1), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Occupied(1), Occupied(0), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(1), Empty, Empty],
                [Empty, Empty, Occupied(0), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
        ]);

        pub(in super::super) const COLLECTIONS: [CellTagTableCollection; 1] = [DOUBLE];
    }

    /// 空でないセル3つからなるブロックの形状を定義するモジュール．
    pub(super) mod triple {
        use super::*;

        /// 一直線形状．
        const BAR: CellTagTableCollection = CellTagTableCollection([
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Occupied(0), Occupied(1), Occupied(2), Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(0), Empty, Empty],
                [Empty, Empty, Occupied(1), Empty, Empty],
                [Empty, Empty, Occupied(2), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Occupied(2), Occupied(1), Occupied(0), Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(2), Empty, Empty],
                [Empty, Empty, Occupied(1), Empty, Empty],
                [Empty, Empty, Occupied(0), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
        ]);

        /// 90度に曲がった形．
        const BENT: CellTagTableCollection = CellTagTableCollection([
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Occupied(2), Empty],
                [Empty, Empty, Occupied(0), Occupied(1), Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(0), Empty, Empty],
                [Empty, Empty, Occupied(1), Occupied(2), Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Occupied(1), Occupied(0), Empty, Empty],
                [Empty, Occupied(2), Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Occupied(2), Occupied(1), Empty, Empty],
                [Empty, Empty, Occupied(0), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
        ]);

        pub(in super::super) const COLLECTIONS: [CellTagTableCollection; 2] = [BAR, BENT];
    }

    pub(super) mod quadruple {
        use super::*;

        /// テトリスでいうOミノと同一の形状．
        const O: CellTagTableCollection = CellTagTableCollection([
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(3), Occupied(2), Empty],
                [Empty, Empty, Occupied(0), Occupied(1), Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(0), Occupied(3), Empty],
                [Empty, Empty, Occupied(1), Occupied(2), Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(1), Occupied(0), Empty],
                [Empty, Empty, Occupied(2), Occupied(3), Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(2), Occupied(1), Empty],
                [Empty, Empty, Occupied(3), Occupied(0), Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
        ]);

        /// Zミノ
        const Z: CellTagTableCollection = CellTagTableCollection([
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Occupied(3), Empty],
                [Empty, Empty, Occupied(0), Occupied(2), Empty],
                [Empty, Empty, Occupied(1), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Occupied(1), Occupied(0), Empty, Empty],
                [Empty, Empty, Occupied(2), Occupied(3), Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(1), Empty, Empty],
                [Empty, Occupied(2), Occupied(0), Empty, Empty],
                [Empty, Occupied(3), Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Occupied(3), Occupied(2), Empty, Empty],
                [Empty, Empty, Occupied(0), Occupied(1), Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
        ]);

        /// Sミノ
        const S: CellTagTableCollection = CellTagTableCollection([
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(1), Empty, Empty],
                [Empty, Empty, Occupied(0), Occupied(2), Empty],
                [Empty, Empty, Empty, Occupied(3), Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(0), Occupied(1), Empty],
                [Empty, Occupied(3), Occupied(2), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Occupied(3), Empty, Empty, Empty],
                [Empty, Occupied(2), Occupied(0), Empty, Empty],
                [Empty, Empty, Occupied(1), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(2), Occupied(3), Empty],
                [Empty, Occupied(1), Occupied(0), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
        ]);

        /// Jミノ
        const J: CellTagTableCollection = CellTagTableCollection([
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(2), Occupied(3), Empty],
                [Empty, Empty, Occupied(1), Empty, Empty],
                [Empty, Empty, Occupied(0), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Occupied(0), Occupied(1), Occupied(2), Empty],
                [Empty, Empty, Empty, Occupied(3), Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(0), Empty, Empty],
                [Empty, Empty, Occupied(1), Empty, Empty],
                [Empty, Occupied(3), Occupied(2), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Occupied(3), Empty, Empty, Empty],
                [Empty, Occupied(2), Occupied(1), Occupied(0), Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
        ]);

        /// Lミノ
        const L: CellTagTableCollection = CellTagTableCollection([
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(0), Empty, Empty],
                [Empty, Empty, Occupied(1), Empty, Empty],
                [Empty, Empty, Occupied(2), Occupied(3), Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Occupied(2), Occupied(1), Occupied(0), Empty],
                [Empty, Occupied(3), Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Occupied(3), Occupied(2), Empty, Empty],
                [Empty, Empty, Occupied(1), Empty, Empty],
                [Empty, Empty, Occupied(0), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Occupied(3), Empty],
                [Empty, Occupied(0), Occupied(1), Occupied(2), Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
        ]);

        /// Tミノ
        const T: CellTagTableCollection = CellTagTableCollection([
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(0), Empty, Empty],
                [Empty, Empty, Occupied(1), Occupied(3), Empty],
                [Empty, Empty, Occupied(2), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Occupied(2), Occupied(1), Occupied(0), Empty],
                [Empty, Empty, Occupied(3), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(2), Empty, Empty],
                [Empty, Occupied(3), Occupied(1), Empty, Empty],
                [Empty, Empty, Occupied(0), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Occupied(3), Empty, Empty],
                [Empty, Occupied(0), Occupied(1), Occupied(2), Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
        ]);

        /// Iミノ
        const I: CellTagTableCollection = CellTagTableCollection([
            [
                [Empty, Empty, Occupied(0), Empty, Empty],
                [Empty, Empty, Occupied(1), Empty, Empty],
                [Empty, Empty, Occupied(2), Empty, Empty],
                [Empty, Empty, Occupied(3), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Occupied(3), Occupied(2), Occupied(1), Occupied(0)],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Occupied(3), Empty, Empty],
                [Empty, Empty, Occupied(2), Empty, Empty],
                [Empty, Empty, Occupied(1), Empty, Empty],
                [Empty, Empty, Occupied(0), Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Occupied(0), Occupied(1), Occupied(2), Occupied(3)],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
        ]);

        pub(in super::super) const COLLECTIONS: [CellTagTableCollection; 7] = [O, Z, S, J, L, T, I];
    }

    pub(super) mod quintuple {
        use super::*;

        const BAR: CellTagTableCollection = CellTagTableCollection([
            [
                [Empty, Empty, Occupied(0), Empty, Empty],
                [Empty, Empty, Occupied(1), Empty, Empty],
                [Empty, Empty, Occupied(2), Empty, Empty],
                [Empty, Empty, Occupied(3), Empty, Empty],
                [Empty, Empty, Occupied(4), Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [
                    Occupied(4),
                    Occupied(3),
                    Occupied(2),
                    Occupied(1),
                    Occupied(0),
                ],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
            [
                [Empty, Empty, Occupied(4), Empty, Empty],
                [Empty, Empty, Occupied(3), Empty, Empty],
                [Empty, Empty, Occupied(2), Empty, Empty],
                [Empty, Empty, Occupied(1), Empty, Empty],
                [Empty, Empty, Occupied(0), Empty, Empty],
            ],
            [
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
                [
                    Occupied(0),
                    Occupied(1),
                    Occupied(2),
                    Occupied(2),
                    Occupied(4),
                ],
                [Empty, Empty, Empty, Empty, Empty],
                [Empty, Empty, Empty, Empty, Empty],
            ],
        ]);

        pub(in super::super) const COLLECTIONS: [CellTagTableCollection; 1] = [BAR];
    }
}

#[cfg(test)]
mod tests_block {
    use super::Cell::{Bomb, Empty, Normal};
    use super::*;

    #[test]
    fn test_cells_without_bomb() {
        let block = Block::new(
            &block_template::quadruple::COLLECTIONS[0],
            Direction::Above,
            BombTag::None,
        );
        assert_eq!([Empty; 5], block.cells[0]);
        assert_eq!([Empty, Empty, Normal, Normal, Empty], block.cells[1]);
        assert_eq!([Empty, Empty, Normal, Normal, Empty], block.cells[2]);
        assert_eq!([Empty; 5], block.cells[3]);
        assert_eq!([Empty; 5], block.cells[4]);
    }

    #[test]
    fn test_cells_all_bomb() {
        let block = Block::new(
            &block_template::quadruple::COLLECTIONS[0],
            Direction::Above,
            BombTag::All,
        );
        assert_eq!([Empty; 5], block.cells[0]);
        assert_eq!([Empty, Empty, Bomb, Bomb, Empty], block.cells[1]);
        assert_eq!([Empty, Empty, Bomb, Bomb, Empty], block.cells[2]);
        assert_eq!([Empty; 5], block.cells[3]);
        assert_eq!([Empty; 5], block.cells[4]);
    }

    #[test]
    fn test_cells_single_bomb() {
        let block = Block::new(
            &block_template::quadruple::COLLECTIONS[0],
            Direction::Above,
            BombTag::Single(1),
        );
        assert_eq!([Empty; 5], block.cells[0]);
        assert_eq!([Empty, Empty, Normal, Bomb, Empty], block.cells[1]);
        assert_eq!([Empty, Empty, Normal, Normal, Empty], block.cells[2]);
        assert_eq!([Empty; 5], block.cells[3]);
        assert_eq!([Empty; 5], block.cells[4]);
    }

    #[test]
    fn test_rotate_clockwise() {
        let block = Block::new(
            &block_template::quadruple::COLLECTIONS[0],
            Direction::Above,
            BombTag::Single(1),
        );
        let block = block.rotate_clockwise();
        assert_eq!(Direction::Above.rotate_clockwise(), block.direction);
        assert_eq!(BombTag::Single(1), block.bomb_tag);
        assert_eq!([Empty; 5], block.cells[0]);
        assert_eq!([Empty, Empty, Normal, Normal, Empty], block.cells[1]);
        assert_eq!([Empty, Empty, Normal, Bomb, Empty], block.cells[2]);
        assert_eq!([Empty; 5], block.cells[3]);
        assert_eq!([Empty; 5], block.cells[4]);
    }

    #[test]
    fn test_rotate_unticlockwise() {
        let block = Block::new(
            &block_template::quadruple::COLLECTIONS[0],
            Direction::Above,
            BombTag::Single(1),
        );
        let block = block.rotate_unticlockwise();
        assert_eq!(Direction::Above.rotate_unticlockwise(), block.direction);
        assert_eq!(BombTag::Single(1), block.bomb_tag);
        assert_eq!([Empty; 5], block.cells[0]);
        assert_eq!([Empty, Empty, Bomb, Normal, Empty], block.cells[1]);
        assert_eq!([Empty, Empty, Normal, Normal, Empty], block.cells[2]);
        assert_eq!([Empty; 5], block.cells[3]);
        assert_eq!([Empty; 5], block.cells[4]);
    }
}
