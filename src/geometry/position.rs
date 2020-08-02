use std::convert::TryInto;
use std::ops::{Add, Sub};

/// 座標や移動量の表現のために利用される型．
pub type Shift = i8;

/// x方向に一次元の長さをもつ格子の座標を表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PositionX {
    /// 原点からこの点までの，右方向を正とした距離．
    pub right_shift: Shift,
}

impl PositionX {
    /// 原点の座標を返す．
    pub const fn origin() -> PositionX {
        Self { right_shift: 0 }
    }

    /// この点の，右向き正とした場合の原点からの位置を返す．
    /// # Returns
    /// 1. この点が原点または正の座標に存在する場合は`Some(position)`を返す．
    /// 1. この点が負の座標に存在する場合は`None`を返す．
    pub fn as_positive_index(&self) -> Option<usize> {
        self.right_shift.try_into().ok()
    }
}

impl Add<MoveX> for PositionX {
    type Output = PositionX;

    fn add(self, rhs: MoveX) -> Self::Output {
        Self {
            right_shift: self.right_shift + rhs.0,
        }
    }
}

impl Sub for PositionX {
    type Output = MoveX;

    fn sub(self, rhs: Self) -> Self::Output {
        MoveX(self.right_shift - rhs.right_shift)
    }
}

/// x方向の移動量を表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MoveX(Shift);

impl Add for MoveX {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for MoveX {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

/// y方向に一次元の長さをもつ格子の座標を表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PositionY {
    /// 原点からこの点までの，下方向を正とした距離．
    pub below_shift: Shift,
}

impl PositionY {
    pub const fn origin() -> PositionY {
        Self { below_shift: 0 }
    }

    /// この点の，下向き正とした場合の原点からの位置を返す．
    /// # Returns
    /// 1. この点が原点または正の座標に存在する場合は`Some(position)`を返す．
    /// 1. この点が負の座標に存在する場合は`None`を返す．
    pub fn as_positive_index(&self) -> Option<usize> {
        self.below_shift.try_into().ok()
    }
}

impl Add<MoveY> for PositionY {
    type Output = Self;

    fn add(self, rhs: MoveY) -> Self::Output {
        Self {
            below_shift: self.below_shift + rhs.0,
        }
    }
}

impl Sub for PositionY {
    type Output = MoveY;

    fn sub(self, rhs: Self) -> Self::Output {
        MoveY(self.below_shift - rhs.below_shift)
    }
}

/// y方向の移動量を表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MoveY(Shift);

impl Add for MoveY {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for MoveY {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

/// フィールドにおけるセルの位置を表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position(pub PositionX, pub PositionY);

impl Position {
    pub const fn origin() -> Position {
        Self(PositionX::origin(), PositionY::origin())
    }

    pub const fn x(&self) -> PositionX {
        self.0
    }

    pub const fn y(&self) -> PositionY {
        self.1
    }
}

impl<T: Into<Movement>> Add<T> for Position {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        let x = self.0 + rhs.0;
        let y = self.1 + rhs.1;
        Self(x, y)
    }
}

impl Sub for Position {
    type Output = Movement;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.0 - rhs.0;
        let y = self.1 - rhs.1;
        Movement(x, y)
    }
}

/// 二次元格子上の移動量を表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Movement(pub MoveX, pub MoveY);

impl Movement {
    pub const fn x(&self) -> MoveX {
        self.0
    }

    pub const fn y(&self) -> MoveY {
        self.1
    }
}

impl From<MoveX> for Movement {
    fn from(x: MoveX) -> Self {
        Self(x, MoveY(0))
    }
}

impl From<MoveY> for Movement {
    fn from(y: MoveY) -> Self {
        Self(MoveX(0), y)
    }
}

impl Add<MoveY> for MoveX {
    type Output = Movement;

    fn add(self, rhs: MoveY) -> Self::Output {
        Movement(self, rhs)
    }
}

impl Add<MoveX> for MoveY {
    type Output = Movement;

    fn add(self, rhs: MoveX) -> Self::Output {
        Movement(rhs, self)
    }
}

impl<T: Into<Movement>> Add<T> for Movement {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        let x = self.0 + rhs.0;
        let y = self.1 + rhs.1;
        Self(x, y)
    }
}

pub const fn right(shift: Shift) -> MoveX {
    MoveX(shift)
}

pub const fn left(shift: Shift) -> MoveX {
    MoveX(-shift)
}

pub const fn below(shift: Shift) -> MoveY {
    MoveY(shift)
}

pub const fn above(shift: Shift) -> MoveY {
    MoveY(-shift)
}

#[cfg(test)]
mod tests_position_x {
    use super::*;

    #[test]
    fn test_origin() {
        let x = PositionX::origin();
        assert_eq!(0, x.right_shift);
    }

    #[test]
    fn test_as_positive_index() {
        assert_eq!(
            Some(2),
            (PositionX::origin() + right(2)).as_positive_index()
        );
        assert_eq!(Some(0), PositionX::origin().as_positive_index());
        assert!((PositionX::origin() + left(1))
            .as_positive_index()
            .is_none());
    }

    #[test]
    fn test_add() {
        assert_eq!(9, (PositionX::origin() + right(9)).right_shift);
    }

    #[test]
    fn test_sub() {
        let p1 = PositionX::origin() + right(10);
        let p2 = PositionX::origin() + right(5);
        assert_eq!(right(5), p1 - p2);
    }
}

#[cfg(test)]
mod tests_move_x {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(right(5), right(2) + right(3));
    }
}

#[cfg(test)]
mod tests_position_y {
    use super::*;

    #[test]
    fn test_origin() {
        let y = PositionY::origin();
        assert_eq!(0, y.below_shift);
    }

    #[test]
    fn test_as_positive_index() {
        assert_eq!(
            Some(2),
            (PositionY::origin() + below(2)).as_positive_index()
        );
        assert_eq!(Some(0), PositionY::origin().as_positive_index());
        assert!((PositionY::origin() + above(1))
            .as_positive_index()
            .is_none());
    }

    #[test]
    fn test_add() {
        assert_eq!(9, (PositionY::origin() + below(9)).below_shift);
    }

    #[test]
    fn test_sub() {
        let p1 = PositionY::origin() + below(10);
        let p2 = PositionY::origin() + below(5);
        assert_eq!(below(5), p1 - p2);
    }
}

#[cfg(test)]
mod tests_move_y {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(below(5), below(2) + below(3));
    }
}

#[cfg(test)]
mod tests_movement {
    use super::*;

    #[test]
    fn test_from_movex() {
        let m = Movement::from(left(1));
        assert_eq!(Movement(left(1), below(0)), m);
    }

    #[test]
    fn test_from_movey() {
        let m = Movement::from(below(1));
        assert_eq!(Movement(left(0), below(1)), m);
    }

    #[test]
    fn test_add() {
        let m1 = Movement(left(1), below(2));
        let m2 = Movement(left(3), below(4));
        assert_eq!(Movement(left(4), below(6)), m1 + m2);
    }
}

#[cfg(test)]
mod tests_position {
    use super::*;

    #[test]
    fn test_origin() {
        let p = Position::origin();
        assert_eq!(PositionX::origin(), p.0);
        assert_eq!(PositionY::origin(), p.1);
    }

    #[test]
    fn test_add() {
        let p = Position::origin();
        let m = Movement(right(5), below(10));
        let p = p + m;
        assert_eq!(PositionX::origin() + right(5), p.0);
        assert_eq!(PositionY::origin() + below(10), p.1);
    }
}
