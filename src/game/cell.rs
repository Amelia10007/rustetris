use crate::geometry::*;
use crate::graphics::*;

/// セルを表す．
/// セルは，ブロックを構成する最小単位である．
/// また，フィールドに二次元格子状に配置されるものでもある．
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    /// 空セル．
    Empty,
    /// 通常のセル．
    Normal,
    /// ボムセル．
    Bomb,
    /// デカボムの左上を表すセル．
    BigBombUpperLeft,
    /// デカボムの右上に割り当てられるセル．
    BigBombUpperRight,
    /// デカボムの左下を表すセル．
    BigBombLowerLeft,
    /// デカボムの右下に割り当てられるセル．
    BigBombLowerRight,
}

impl Cell {
    /// このセルが空セルであるか返す．
    pub fn is_empty(&self) -> bool {
        match self {
            Cell::Empty => true,
            _ => false,
        }
    }
}

impl Cell {
    fn char_for_display(&self) -> SquareChar {
        use Cell::*;

        match self {
            Empty => SquareChar::new(' ', '.'),
            Normal => SquareChar::new('[', ']'),
            Bomb => SquareChar::new('[', ']'),
            BigBombUpperLeft | BigBombUpperRight | BigBombLowerLeft | BigBombLowerRight => {
                SquareChar::new('=', '=')
            }
        }
    }

    fn color_for_display(&self) -> CanvasCellColor {
        use Cell::*;
        use Color::*;

        match self {
            Empty => CanvasCellColor::new(White, Black),
            Normal => CanvasCellColor::new(Cyan, Black),
            Bomb => CanvasCellColor::new(Red, Black),
            BigBombUpperLeft | BigBombUpperRight | BigBombLowerLeft | BigBombLowerRight => {
                CanvasCellColor::new(White, Red)
            }
        }
    }

    fn canvas_cell(&self) -> CanvasCell {
        CanvasCell::new(self.char_for_display(), self.color_for_display())
    }
}

impl Drawable for Cell {
    fn region_size(&self) -> Movement {
        right(1) + below(1)
    }

    fn draw<C: Canvas>(&self, canvas: &mut C) {
        canvas.draw_cell(Pos::origin(), self.canvas_cell());
    }
}

#[cfg(test)]
mod tests {
    use super::Cell::*;

    #[test]
    fn test_is_empty() {
        assert!(Empty.is_empty());
        assert!(!Normal.is_empty());
        assert!(!Bomb.is_empty());
        assert!(!BigBombUpperLeft.is_empty());
        assert!(!BigBombUpperRight.is_empty());
        assert!(!BigBombLowerLeft.is_empty());
        assert!(!BigBombLowerRight.is_empty());
    }
}
