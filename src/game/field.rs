use super::cell::Cell;
use crate::geometry::*;
use std::ops::{Deref, DerefMut};

mod consts {
    pub const WIDTH: usize = 10;
    pub const HEIGHT: usize = 20;
}

use consts::*;

/// セルの集合として表されるフィールド．
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field {
    /// 各位置に割り当てられたセル．
    cells: [[Cell; WIDTH]; HEIGHT],
}

impl Field {
    /// 空のフィールドを返す．
    /// # Returns
    /// すべてのセルが`Cell::Empty`である`Field`．
    pub const fn empty() -> Field {
        Self {
            cells: [[Cell::Empty; WIDTH]; HEIGHT],
        }
    }

    /// 指定した位置のセルへの参照を返す．
    /// # Returns
    /// 1. 指定した位置にセルが存在する場合は`Some(cell)`を返す．
    /// 1. 指定した位置にセルが存在しない場合は`None`を返す．
    pub fn get(&self, p: Position) -> Option<&Cell> {
        let x = p.x().as_positive_index()?;
        let y = p.y().as_positive_index()?;
        self.cells.get(y).and_then(|row| row.get(x))
    }

    /// 指定した位置のセルへの可変参照を返す．
    /// # Returns
    /// 1. 指定した位置にセルが存在する場合は`Some(cell)`を返す．
    /// 1. 指定した位置にセルが存在しない場合は`None`を返す．
    pub fn get_mut(&mut self, p: Position) -> Option<&mut Cell> {
        let x = p.x().as_positive_index()?;
        let y = p.y().as_positive_index()?;
        self.cells.get_mut(y).and_then(|row| row.get_mut(x))
    }

    /// 指定した位置のライン(同じy座標をもつセル列)を返す．
    /// # Returns
    /// 1. 指定した位置にラインが存在する場合は`Some(row)`を返す．
    /// 1. 指定した位置にラインが存在しない場合は`None`を返す．
    pub fn row(&self, y: PositionY) -> Option<FieldRow<'_>> {
        match y.as_positive_index() {
            Some(y_index) if y_index < HEIGHT => Some(FieldRow::from_y_index(self, y_index)),
            _ => None,
        }
    }

    /// 指定した位置の可変ライン(同じy座標をもつセル列)を返す．
    /// # Returns
    /// 1. 指定した位置にラインが存在する場合は`Some(row)`を返す．
    /// 1. 指定した位置にラインが存在しない場合は`None`を返す．
    pub fn row_mut(&mut self, y: PositionY) -> Option<FieldRowMut<'_>> {
        FieldRowMut::new(self, y)
    }

    /// 最上段から順にこのフィールドのラインを返す．
    pub fn rows(&self) -> impl Iterator<Item = FieldRow<'_>> + '_ {
        (0..HEIGHT).map(move |i| FieldRow::from_y_index(self, i))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldRow<'f> {
    field: &'f Field,
    y_index: usize,
}

impl<'f> FieldRow<'f> {
    pub fn y(&self) -> PositionY {
        PositionY::origin() + below(self.y_index as i8)
    }

    fn from_y_index(field: &'f Field, y_index: usize) -> FieldRow<'f> {
        debug_assert!(y_index < HEIGHT);
        Self { field, y_index }
    }
}

impl Deref for FieldRow<'_> {
    type Target = [Cell];

    fn deref(&self) -> &Self::Target {
        &self.field.cells[self.y_index]
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FieldRowMut<'f> {
    field: &'f mut Field,
    y_index: usize,
}

impl<'f> FieldRowMut<'f> {
    pub fn new(field: &'f mut Field, y: PositionY) -> Option<FieldRowMut<'f>> {
        match y.as_positive_index() {
            Some(y_index) if y_index < HEIGHT => Some(Self { field, y_index }),
            _ => None,
        }
    }

    pub fn y(&self) -> PositionY {
        PositionY::origin() + below(self.y_index as i8)
    }
}

impl Deref for FieldRowMut<'_> {
    type Target = [Cell];

    fn deref(&self) -> &Self::Target {
        &self.field.cells[self.y_index]
    }
}

impl DerefMut for FieldRowMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.field.cells[self.y_index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let field = Field::empty();
        for row in field.cells.iter() {
            for &cell in row.iter() {
                assert_eq!(Cell::Empty, cell);
            }
        }
    }

    #[test]
    fn test_get() {
        let field = Field::empty();

        // 原点座標(左上)にセルは存在するはず
        let p = Position(PositionX::origin(), PositionY::origin());
        assert_eq!(Some(&Cell::Empty), field.get(p));
        // 右上
        let upper_right = p + right(WIDTH as i8 - 1);
        assert_eq!(Some(&Cell::Empty), field.get(upper_right));
        // 左下
        let lower_left = p + below(HEIGHT as i8 - 1);
        assert_eq!(Some(&Cell::Empty), field.get(lower_left));
        // 右下
        let lower_right = p + right(WIDTH as i8 - 1) + below(HEIGHT as i8 - 1);
        assert_eq!(Some(&Cell::Empty), field.get(lower_right));

        // 正のx方向にはみ出た座標
        let outer_positive_x = p + right(WIDTH as i8);
        assert!(field.get(outer_positive_x).is_none());
        // 負のx方向にはみ出た座標
        let outer_negative_x = p + left(1);
        assert!(field.get(outer_negative_x).is_none());
        // 正のy方向にはみ出た座標
        let outer_positive_y = p + below(HEIGHT as i8);
        assert!(field.get(outer_positive_y).is_none());
        // 負のy方向にはみ出た座標
        let outer_negative_y = p + above(1);
        assert!(field.get(outer_negative_y).is_none());
    }

    #[test]
    fn test_get_mut() {
        let mut field = Field::empty();

        // 原点座標(左上)にセルは存在するはず
        let p = Position(PositionX::origin(), PositionY::origin());
        *field.get_mut(p).unwrap() = Cell::Normal;
        assert_eq!(Some(&Cell::Normal), field.get(p));
        // 右上
        let upper_right = p + right(WIDTH as i8 - 1);
        *field.get_mut(upper_right).unwrap() = Cell::Bomb;
        assert_eq!(Some(&Cell::Bomb), field.get(upper_right));
        // 左下
        let lower_left = p + below(HEIGHT as i8 - 1);
        *field.get_mut(lower_left).unwrap() = Cell::BigBombUpperLeft;
        assert_eq!(Some(&Cell::BigBombUpperLeft), field.get(lower_left));
        // 右下
        let lower_right = p + right(WIDTH as i8 - 1) + below(HEIGHT as i8 - 1);
        *field.get_mut(lower_right).unwrap() = Cell::BigBombPart;
        assert_eq!(Some(&Cell::BigBombPart), field.get(lower_right));

        // 正のx方向にはみ出た座標
        let outer_positive_x = p + right(WIDTH as i8);
        assert!(field.get_mut(outer_positive_x).is_none());
        // 負のx方向にはみ出た座標
        let outer_negative_x = p + left(1);
        assert!(field.get_mut(outer_negative_x).is_none());
        // 正のy方向にはみ出た座標
        let outer_positive_y = p + below(HEIGHT as i8);
        assert!(field.get_mut(outer_positive_y).is_none());
        // 負のy方向にはみ出た座標
        let outer_negative_y = p + above(1);
        assert!(field.get_mut(outer_negative_y).is_none());
    }

    #[test]
    fn test_row() {
        let field = Field::empty();

        let upper_row = field.row(PositionY::origin()).unwrap();
        assert_eq!(PositionY::origin(), upper_row.y());
        assert_eq!(WIDTH, upper_row.len());

        let lower_row = field
            .row(PositionY::origin() + below(HEIGHT as i8 - 1))
            .unwrap();
        assert_eq!(PositionY::origin() + below(HEIGHT as i8 - 1), lower_row.y());
        assert_eq!(WIDTH, lower_row.len());

        // 上方向にはみ出し
        assert!(field.row(PositionY::origin() + above(1)).is_none());
        // 下方向にはみ出し
        assert!(field
            .row(PositionY::origin() + below(HEIGHT as i8))
            .is_none());
    }

    #[test]
    fn test_row_mut() {
        let mut field = Field::empty();

        {
            let mut upper_row = field.row_mut(PositionY::origin()).unwrap();
            assert_eq!(PositionY::origin(), upper_row.y());
            assert_eq!(WIDTH, upper_row.len());
            for cell in upper_row.iter_mut() {
                *cell = Cell::Bomb;
            }
        }
        assert!(field
            .row(PositionY::origin())
            .unwrap()
            .iter()
            .all(|&cell| cell == Cell::Bomb));

        assert!(field.row_mut(PositionY::origin() + above(1)).is_none());
        assert!(field
            .row_mut(PositionY::origin() + below(HEIGHT as i8))
            .is_none());
    }

    #[test]
    fn test_rows() {
        let field = Field::empty();
        let rows = field.rows().collect::<Vec<_>>();
        assert_eq!(HEIGHT, rows.len());

        for (i, row) in rows.into_iter().enumerate() {
            let y = PositionY::origin() + below(i as i8);
            assert_eq!(y, row.y());
            let row2 = field.row(y).unwrap();
            assert_eq!(row2, row);
        }
    }
}