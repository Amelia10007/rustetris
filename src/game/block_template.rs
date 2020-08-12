use super::Cell;
use crate::geometry::*;
use crate::graphics::*;
use std::ops::Index;

mod consts {
    /// ブロックの形状を定義するテーブルの一片の長さ．
    pub const BLOCK_TABLE_SIZE: usize = 5;
    /// ブロックの向きの数．
    pub const ROTATION_KIND: usize = 4;
}

use consts::*;

/// ブロックの形状を表すタグ．
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockShape {
    Single(SingleBlockShape),
    Double(DoubleBlockShape),
    Triple(TripleBlockShape),
    Quadruple(QuadrupleBlockShape),
    Quintuple(QuintupleBlockShape),
}

impl BlockShape {
    /// このブロック形状が，空でないセルをいくつ含むか返す．
    pub fn non_empty_cell_count(&self) -> usize {
        match self {
            BlockShape::Single(_) => 1,
            BlockShape::Double(_) => 2,
            BlockShape::Triple(_) => 3,
            BlockShape::Quadruple(_) => 4,
            BlockShape::Quintuple(_) => 5,
        }
    }
}

/// 空でないセル1つからなるブロック形状．
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SingleBlockShape {
    /// セルひとつからなる形状．
    O,
}

impl Into<BlockShape> for SingleBlockShape {
    fn into(self) -> BlockShape {
        BlockShape::Single(self)
    }
}

/// 空でないセル2つからなるブロック形状．
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DoubleBlockShape {
    /// 2つのセルが連続している．
    ShortI,
}

impl Into<BlockShape> for DoubleBlockShape {
    fn into(self) -> BlockShape {
        BlockShape::Double(self)
    }
}

/// 空でないセル3つからなるブロック形状．
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TripleBlockShape {
    /// 3つのセルが連続している．
    ShortI,
    /// Lミノの長辺からセルを1つ取り除いた形状．
    ShortL,
    /// Jミノの長辺からセルを1つ取り除いた形状．
    ShortJ,
}

impl Into<BlockShape> for TripleBlockShape {
    fn into(self) -> BlockShape {
        BlockShape::Triple(self)
    }
}

/// 空でないセル4つからなるブロック形状．
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuadrupleBlockShape {
    /// Oミノ．
    O,
    /// Lミノ．
    L,
    /// Jミノ．
    J,
    /// Zミノ．
    Z,
    /// Sミノ．
    S,
    /// Tミノ．
    T,
    /// Iミノ．
    I,
}

impl Into<BlockShape> for QuadrupleBlockShape {
    fn into(self) -> BlockShape {
        BlockShape::Quadruple(self)
    }
}

/// 空でないセル5つからなるブロック形状．
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuintupleBlockShape {
    /// Iミノを長くした形状．
    LongI,
    /// Lミノの長辺先端にセルを1つ追加した形状．
    LongL,
    /// Jミノの長辺先端にセルを1つ追加した形状．
    LongJ,
    /// Lミノの短辺先端にセルを1つ追加した形状．
    LargeL,
    /// Jミノの短辺先端にセルを1つ追加した形状．
    LargeJ,
    /// 上向きのTミノの左端にセルを1つ追加した形状．
    LongTLeft,
    /// 上向きのTミノの右端にセルを1つ追加した形状．
    LongTRight,
    /// 上向きのTミノの上端にセルを1つ追加した形状．
    LargeT,
    /// 上向きのTミノの中央下にセルを1つ追加した形状．
    Star,
    /// Oミノの左上にセルを1つ追加した形状．
    OUpperLeft,
    /// Oミノの左下にセルを1つ追加した形状．
    OLowerLeft,
    /// Zミノの末尾にセルを1つ追加した形状．
    LongZ,
    /// Sミノの末尾にセルを1つ追加した形状．
    LongS,
    /// Zミノの中心にセルを1つ追加した形状．
    LargeZ,
    /// Sミノの中心にセルを1つ追加した形状．
    LargeS,
    /// 右向きのTミノとJミノを合わせた形状．
    JT,
    /// 右向きのTミノとLミノを合わせた形状．
    LT,
}

impl Into<BlockShape> for QuintupleBlockShape {
    fn into(self) -> BlockShape {
        BlockShape::Quintuple(self)
    }
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

/// ブロック形状テンプレートに利用するタグ．
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CellTag {
    /// このセルが空でないことを表す．
    /// また，0から始まる互いに異なるラベルが関連付けられる．
    Occupied(usize),
    /// このセルが空であることを表す．
    Empty,
}

/// ブロック生成ルールを表す．
pub trait BlockSelector {
    /// ブロックの形状を返す．
    fn select_block_shape(&mut self) -> BlockShape;

    /// ボムセルの数および位置を返す．
    fn select_bomb(&mut self, shape: BlockShape) -> BombTag;

    /// ブロックを生成して返す．
    fn generate_block(&mut self) -> Block {
        let shape = self.select_block_shape();
        let bomb = self.select_bomb(shape);
        // 形状に合致したテーブルを取得
        let table = block_template::get_cell_tag_collection(shape);
        // テーブルをブロックに変換
        Block::new(table, Direction::Above, bomb)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    /// このブロックを構成するセル．
    cells: Table<Cell>,
    /// このブロックを生成するために利用した形状テーブル．
    /// ブロックの回転処理に利用される．
    tables: &'static CellTagTableCollection,
    /// このブロックの方向．
    /// ブロックの回転処理に利用される．
    direction: Direction,
    /// このブロックのボムセルの数や位置を決定する．
    bomb_tag: BombTag,
}

impl Block {
    /// ブロックを生成して返す．
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

    /// このブロックを構成する，空でないすべてのセルとその位置を列挙する．
    pub fn iter_pos_and_occupied_cell(&self) -> impl Iterator<Item = (Pos, &Cell)> + '_ {
        self.cells.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, cell)| {
                    let p = Pos(PosX::right(x as i8), PosY::below(y as i8));
                    (p, cell)
                })
                .filter(|(_, &cell)| cell != Cell::Empty)
        })
    }

    /// このブロックを構成する，すべてのセルとその位置を列挙する．
    pub fn iter_pos_and_cell(&self) -> impl Iterator<Item = (Pos, &Cell)> + '_ {
        self.cells.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, cell)| {
                let p = Pos(PosX::right(x as i8), PosY::below(y as i8));
                (p, cell)
            })
        })
    }

    /// このブロックを時計回りに90度回転させたブロックを返す．
    pub fn rotate_clockwise(&self) -> Block {
        let direction = self.direction.rotate_clockwise();
        Self::new(self.tables, direction, self.bomb_tag)
    }

    /// このブロックを反時計回りに90度回転させたブロックを返す．
    pub fn rotate_unticlockwise(&self) -> Block {
        let direction = self.direction.rotate_unticlockwise();
        Self::new(self.tables, direction, self.bomb_tag)
    }

    /// 指定した条件に合致したセルテーブルを返す．
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

impl Default for Block {
    fn default() -> Self {
        let tables = block_template::get_cell_tag_collection(SingleBlockShape::O.into());
        Self::new(tables, Direction::Above, BombTag::None)
    }
}

impl Drawable for Block {
    fn region_size(&self) -> Movement {
        right(BLOCK_TABLE_SIZE as i8) + below(BLOCK_TABLE_SIZE as i8)
    }

    fn draw<C: Canvas>(&self, canvas: &mut C) {
        for (pos, cell) in self.iter_pos_and_occupied_cell() {
            cell.draw_on_child(pos, canvas);
        }
    }
}

/// ブロックの形状を定義するテンプレートを詰めたモジュール．
mod block_template {
    use super::BlockShape;
    use super::CellTag;
    use super::CellTagTableCollection;
    use super::{
        DoubleBlockShape, QuadrupleBlockShape, QuintupleBlockShape, SingleBlockShape,
        TripleBlockShape,
    };
    use lazy_static::lazy_static;
    use std::collections::HashMap;

    /// 文字数を短くそろえるためだけの定数．
    const EM: CellTag = CellTag::Empty;
    const O0: CellTag = CellTag::Occupied(0);
    const O1: CellTag = CellTag::Occupied(1);
    const O2: CellTag = CellTag::Occupied(2);
    const O3: CellTag = CellTag::Occupied(3);
    const O4: CellTag = CellTag::Occupied(4);

    lazy_static! {
        static ref SINGLE_CELL_TAG_COLLECTION: HashMap<SingleBlockShape, CellTagTableCollection> = {
            let mut map = HashMap::new();
            let o = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);
            map.insert(SingleBlockShape::O, o);
            map
        };
        static ref DOUBLE_CELL_TAG_COLLECTION: HashMap<DoubleBlockShape, CellTagTableCollection> = {
            let mut map = HashMap::new();
            let short_i = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O0, O1, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, O1, O0, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);
            map.insert(DoubleBlockShape::ShortI, short_i);
            map
        };
        static ref TRIPLE_CELL_TAG_COLLECTION: HashMap<TripleBlockShape, CellTagTableCollection> = {
            let mut map = HashMap::new();
            let short_i = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, O0, O1, O2, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, O2, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, O2, O1, O0, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O2, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let short_l = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O2, EM, EM],
                    [EM, EM, O0, O1, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O0, O2, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, O1, O0, EM, EM],
                    [EM, EM, O2, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, O2, O0, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let short_j = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, O2, EM],
                    [EM, EM, O0, O1, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, EM, O1, O2, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, O1, O0, EM, EM],
                    [EM, O2, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, O2, O1, EM, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            map.insert(TripleBlockShape::ShortI, short_i);
            map.insert(TripleBlockShape::ShortL, short_l);
            map.insert(TripleBlockShape::ShortJ, short_j);

            map
        };
        static ref QUADRUPLE_CELL_TAG_COLLECTION: HashMap<QuadrupleBlockShape, CellTagTableCollection> = {
            let mut map = HashMap::new();

            let o = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O3, O2, EM],
                    [EM, EM, O0, O1, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O0, O3, EM],
                    [EM, EM, O1, O2, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O1, O0, EM],
                    [EM, EM, O2, O3, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O2, O1, EM],
                    [EM, EM, O3, O0, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let z = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, O3, EM],
                    [EM, EM, O0, O2, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, O1, O0, EM, EM],
                    [EM, EM, O2, O3, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, O2, O0, EM, EM],
                    [EM, O3, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, O3, O2, EM, EM],
                    [EM, EM, O0, O1, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let s = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, O0, O2, EM],
                    [EM, EM, EM, O3, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O0, O1, EM],
                    [EM, O3, O2, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, O3, EM, EM, EM],
                    [EM, O2, O0, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O2, O3, EM],
                    [EM, O1, O0, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let j = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O2, O3, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, O0, O1, O2, EM],
                    [EM, EM, EM, O3, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, O3, O2, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, O3, EM, EM, EM],
                    [EM, O2, O1, O0, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let l = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, O2, O3, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, O2, O1, O0, EM],
                    [EM, O3, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, O3, O2, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, O3, EM],
                    [EM, O0, O1, O2, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let t = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, EM, O1, O3, EM],
                    [EM, EM, O2, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, O2, O1, O0, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O2, EM, EM],
                    [EM, O3, O1, EM, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, O0, O1, O2, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let i = CellTagTableCollection([
                [
                    [EM, EM, O0, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, O2, EM, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, O3, O2, O1, O0],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, O3, EM, EM],
                    [EM, EM, O2, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, O0, O1, O2, O3],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            map.insert(QuadrupleBlockShape::O, o);
            map.insert(QuadrupleBlockShape::Z, z);
            map.insert(QuadrupleBlockShape::S, s);
            map.insert(QuadrupleBlockShape::J, j);
            map.insert(QuadrupleBlockShape::L, l);
            map.insert(QuadrupleBlockShape::T, t);
            map.insert(QuadrupleBlockShape::I, i);

            map
        };
        static ref QUINTUPLE_CELL_TAG_COLLECTION: HashMap<QuintupleBlockShape, CellTagTableCollection> = {
            let mut map = HashMap::new();

            let long_i = CellTagTableCollection([
                [
                    [EM, EM, O0, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, O2, EM, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, EM, O4, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [O4, O3, O2, O1, O0],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, O4, EM, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, EM, O2, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, O0, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [O0, O1, O2, O3, O4],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let long_l = CellTagTableCollection([
                [
                    [EM, EM, EM, O0, EM],
                    [EM, EM, EM, O1, EM],
                    [EM, EM, EM, O2, EM],
                    [EM, EM, EM, O3, O4],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, O3, O2, O1, O0],
                    [EM, O4, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O4, O3, EM],
                    [EM, EM, EM, O2, EM],
                    [EM, EM, EM, O1, EM],
                    [EM, EM, EM, O0, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, O4],
                    [EM, O0, O1, O2, O3],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let long_j = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O4, O3, EM],
                    [EM, EM, EM, O2, EM],
                    [EM, EM, EM, O1, EM],
                    [EM, EM, EM, O0, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, O4],
                    [EM, O0, O1, O2, O3],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, O0, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, O2, EM, EM],
                    [EM, EM, O3, O4, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, O3, O2, O1, O0],
                    [EM, O4, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let large_l = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O2, O1, O0],
                    [EM, EM, O3, EM, EM],
                    [EM, EM, O4, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, O4, O3, O2, EM],
                    [EM, EM, EM, O1, EM],
                    [EM, EM, EM, O0, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, O4, EM],
                    [EM, EM, EM, O3, EM],
                    [EM, O0, O1, O2, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, O2, O3, O4],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let large_j = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O4, EM, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, EM, O2, O1, O0],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O2, O3, O4],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, O0, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, O0, O1, O2, EM],
                    [EM, EM, EM, O3, EM],
                    [EM, EM, EM, O4, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, O0, EM],
                    [EM, EM, EM, O1, EM],
                    [EM, O4, O3, O2, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let long_t_left = CellTagTableCollection([
                [
                    [EM, EM, O0, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, O2, O4, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, O3, O2, O1, O0],
                    [EM, EM, O4, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, O4, O2, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, O0, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O4, EM, EM],
                    [O0, O1, O2, O3, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let long_t_right = CellTagTableCollection([
                [
                    [EM, EM, O0, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, O4, O2, EM, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O4, EM, EM],
                    [EM, O3, O2, O1, O0],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, EM, O2, O4, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, O0, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [O0, O1, O2, O3, EM],
                    [EM, EM, EM, O4, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let large_t = CellTagTableCollection([
                [
                    [EM, O0, EM, EM, EM],
                    [EM, O1, O3, O4, EM],
                    [EM, O2, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, O2, O1, O0, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, EM, O4, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, O2, EM],
                    [EM, O4, O3, O1, EM],
                    [EM, EM, EM, O0, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, O4, EM, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, O0, O1, O2, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let star = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, O4, O1, O3, EM],
                    [EM, EM, O2, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O4, EM, EM],
                    [EM, O2, O1, O0, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O2, EM, EM],
                    [EM, O3, O1, O4, EM],
                    [EM, EM, O0, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, O0, O1, O2, EM],
                    [EM, EM, O4, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let o_upper_left = CellTagTableCollection([
                [
                    [EM, EM, EM, O4, EM],
                    [EM, EM, O0, O3, EM],
                    [EM, EM, O1, O2, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O1, O0, EM],
                    [EM, EM, O2, O3, O4],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O2, O1, EM],
                    [EM, EM, O3, O0, EM],
                    [EM, EM, O4, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, O4, O3, O2, EM],
                    [EM, EM, O0, O1, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let o_lower_left = CellTagTableCollection([
                [
                    [EM, EM, O4, EM, EM],
                    [EM, EM, O3, O0, EM],
                    [EM, EM, O2, O1, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O2, O3, O4],
                    [EM, EM, O1, O0, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O1, O2, EM],
                    [EM, EM, O0, O3, EM],
                    [EM, EM, EM, O4, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O0, O1, EM],
                    [EM, O4, O3, O2, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let long_z = CellTagTableCollection([
                [
                    [EM, EM, O0, EM, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, O3, O2, EM, EM],
                    [EM, O4, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, O4, O3, EM, EM],
                    [EM, EM, O2, O1, O0],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, O4, EM],
                    [EM, EM, O2, O3, EM],
                    [EM, EM, O1, EM, EM],
                    [EM, EM, O0, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [O0, O1, O2, EM, EM],
                    [EM, EM, O3, O4, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let long_s = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, O0, EM, EM, EM],
                    [EM, O1, O2, EM, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, EM, O4, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O1, O0, EM],
                    [O4, O3, O2, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, O4, EM, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, EM, O2, O1, EM],
                    [EM, EM, EM, O0, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O2, O3, O4],
                    [EM, O0, O1, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let large_z = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, O0, O1, EM, EM],
                    [EM, EM, O2, EM, EM],
                    [EM, EM, O3, O4, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, O0, EM],
                    [EM, O3, O2, O1, EM],
                    [EM, O4, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, O4, O3, EM, EM],
                    [EM, EM, O2, EM, EM],
                    [EM, EM, O1, O0, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, O4, EM],
                    [EM, O1, O2, O3, EM],
                    [EM, O0, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let large_s = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O3, O4, EM],
                    [EM, EM, O2, EM, EM],
                    [EM, O0, O1, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, O0, EM, EM, EM],
                    [EM, O1, O2, O3, EM],
                    [EM, EM, EM, O4, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O1, O0, EM],
                    [EM, EM, O2, EM, EM],
                    [EM, O4, O3, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, O4, EM, EM, EM],
                    [EM, O3, O2, O1, EM],
                    [EM, EM, EM, O0, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let jt = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, O0, EM, EM, EM],
                    [EM, O1, O2, O3, EM],
                    [EM, EM, O4, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O1, O0, EM],
                    [EM, O4, O2, EM, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O4, EM, EM],
                    [EM, O3, O2, O1, EM],
                    [EM, EM, EM, O0, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, EM, O2, O4, EM],
                    [EM, O0, O1, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            let lt = CellTagTableCollection([
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O4, EM, EM],
                    [EM, O1, O2, O3, EM],
                    [EM, O0, EM, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, O0, O1, EM, EM],
                    [EM, EM, O2, O4, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, EM, O0, EM],
                    [EM, O3, O2, O1, EM],
                    [EM, EM, O4, EM, EM],
                    [EM, EM, EM, EM, EM],
                ],
                [
                    [EM, EM, EM, EM, EM],
                    [EM, EM, O3, EM, EM],
                    [EM, O4, O2, EM, EM],
                    [EM, EM, O1, O0, EM],
                    [EM, EM, EM, EM, EM],
                ],
            ]);

            map.insert(QuintupleBlockShape::LongI, long_i);
            map.insert(QuintupleBlockShape::LongL, long_l);
            map.insert(QuintupleBlockShape::LongJ, long_j);
            map.insert(QuintupleBlockShape::LargeL, large_l);
            map.insert(QuintupleBlockShape::LargeJ, large_j);
            map.insert(QuintupleBlockShape::LongTLeft, long_t_left);
            map.insert(QuintupleBlockShape::LongTRight, long_t_right);
            map.insert(QuintupleBlockShape::LargeT, large_t);
            map.insert(QuintupleBlockShape::Star, star);
            map.insert(QuintupleBlockShape::OUpperLeft, o_upper_left);
            map.insert(QuintupleBlockShape::OLowerLeft, o_lower_left);
            map.insert(QuintupleBlockShape::LongZ, long_z);
            map.insert(QuintupleBlockShape::LongS, long_s);
            map.insert(QuintupleBlockShape::LargeZ, large_z);
            map.insert(QuintupleBlockShape::LargeS, large_s);
            map.insert(QuintupleBlockShape::JT, jt);
            map.insert(QuintupleBlockShape::LT, lt);

            map
        };
    }

    /// 指定したブロック形状に対応する形状定義テンプレートを返す．
    pub(super) fn get_cell_tag_collection(shape: BlockShape) -> &'static CellTagTableCollection {
        match shape {
            BlockShape::Single(s) => &SINGLE_CELL_TAG_COLLECTION[&s],
            BlockShape::Double(s) => &DOUBLE_CELL_TAG_COLLECTION[&s],
            BlockShape::Triple(s) => &TRIPLE_CELL_TAG_COLLECTION[&s],
            BlockShape::Quadruple(s) => &QUADRUPLE_CELL_TAG_COLLECTION[&s],
            BlockShape::Quintuple(s) => &QUINTUPLE_CELL_TAG_COLLECTION[&s],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cell::{Bomb, Empty, Normal};
    use super::*;

    #[test]
    fn test_cells_without_bomb() {
        let block = Block::new(
            block_template::get_cell_tag_collection(QuadrupleBlockShape::O.into()),
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
            block_template::get_cell_tag_collection(QuadrupleBlockShape::O.into()),
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
            block_template::get_cell_tag_collection(QuadrupleBlockShape::O.into()),
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
    fn test_iter_pos_and_occupied_cell() {
        let block = Block::new(
            block_template::get_cell_tag_collection(QuadrupleBlockShape::O.into()),
            Direction::Above,
            BombTag::Single(1),
        );
        let mut iter = block.iter_pos_and_occupied_cell();
        assert_eq!(
            (Pos(PosX::right(2), PosY::below(1)), &Cell::Normal),
            iter.next().unwrap()
        );
        assert_eq!(
            (Pos(PosX::right(3), PosY::below(1)), &Cell::Bomb),
            iter.next().unwrap()
        );
        assert_eq!(
            (Pos(PosX::right(2), PosY::below(2)), &Cell::Normal),
            iter.next().unwrap()
        );
        assert_eq!(
            (Pos(PosX::right(3), PosY::below(2)), &Cell::Normal),
            iter.next().unwrap()
        );
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_rotate_clockwise() {
        let block = Block::new(
            block_template::get_cell_tag_collection(QuadrupleBlockShape::O.into()),
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
            block_template::get_cell_tag_collection(QuadrupleBlockShape::O.into()),
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
