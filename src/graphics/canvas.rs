use super::*;
use crate::geometry::*;
pub use console::Color;
use itertools::Itertools;

mod consts {
    pub const CANVAS_WIDTH: usize = 40;
    pub const CANVAS_HEIHGT: usize = 24;
}

use consts::*;

/// キャンバスを表す．
pub trait Canvas {
    /// このキャンバス上の指定した位置に，指定したセルを描画する．
    /// # Returns
    /// キャンバスにセルを描画できた場合は`Some(...)`を返す．
    /// 指定した位置がこのキャンバスの範囲外であった場合は，キャンバスの内容は変更されず，このメソッドは`None`を返す．
    fn draw_cell(&mut self, pos: Pos, cell: CanvasCell) -> Option<()>;

    /// このキャンバスから指定した注目領域を切り抜き，子キャンバスとして返す．
    /// このキャンバスと子キャンバスは状態を共有する．
    /// 子キャンバス上のセルを描画すると，それに対応したこのキャンバスのセルも変更される．
    fn child(&mut self, roi: RegionOfInterest) -> ChildCanvas<'_>;
}

/// 画面全体を描画対象とするキャンバスを表す．
pub struct RootCanvas {
    /// 格子状に配置されたセル．
    cells: [[CanvasCell; CANVAS_WIDTH]; CANVAS_HEIHGT],
}

impl RootCanvas {
    pub fn new() -> RootCanvas {
        Self {
            cells: [[CanvasCell::default(); CANVAS_WIDTH]; CANVAS_HEIHGT],
        }
    }

    /// キャンバス上のすべてのセルを既定の状態にする．
    pub fn clear(&mut self) {
        self.cells = [[CanvasCell::default(); CANVAS_WIDTH]; CANVAS_HEIHGT];
    }

    /// 標準出力にこのキャンバスの内容を表示するための文字列を生成する．
    pub fn construct_output_string(&self, buffer: &mut String) {
        // まずは既存の内容を全消し
        buffer.clear();

        // 1行ずつ
        for row in self.cells.iter() {
            // 行内で連続して同じ色となっているセルをまとめて書き出す．
            // これにより，出力文字数を減らせる．
            for (color, group) in row.iter().group_by(|cell| cell.color).into_iter() {
                let s: String = group.flat_map(|cell| cell.c.0.iter()).collect();
                let content = color.as_style().apply_to(s);
                buffer.push_str(&format!("{}", content));
            }
            // 次の行へ
            buffer.push('\n');
        }
    }
}

impl Canvas for RootCanvas {
    fn draw_cell(&mut self, pos: Pos, cell: CanvasCell) -> Option<()> {
        let y = pos.y().as_positive_index()?;
        let x = pos.x().as_positive_index()?;
        let c = self.cells.get_mut(y).and_then(|row| row.get_mut(x))?;
        *c = cell;
        Some(())
    }

    fn child(&mut self, roi: RegionOfInterest) -> ChildCanvas<'_> {
        ChildCanvas::new(self, roi)
    }
}

/// 子キャンバスを表す．
pub struct ChildCanvas<'root> {
    /// 親となるキャンバス．
    root_canvas: &'root mut RootCanvas,
    /// 親キャンバスの座標系における，この子キャンバスのROI．
    roi: RegionOfInterest,
}

impl<'root> ChildCanvas<'root> {
    pub fn new(root_canvas: &'root mut RootCanvas, roi: RegionOfInterest) -> ChildCanvas<'root> {
        Self { root_canvas, roi }
    }
}

impl<'root> Canvas for ChildCanvas<'root> {
    fn draw_cell(&mut self, pos: Pos, cell: CanvasCell) -> Option<()> {
        let diff = pos - Pos::origin();
        let root_canvas_pos = self.roi.left_top + diff;
        if self.roi.contains(root_canvas_pos) {
            self.root_canvas.draw_cell(root_canvas_pos, cell)
        } else {
            None
        }
    }

    fn child(&mut self, roi: RegionOfInterest) -> ChildCanvas<'_> {
        let left_top = self.roi.left_top + (roi.left_top - Pos::origin());
        let roi = RegionOfInterest::new(left_top, roi.size);
        ChildCanvas::new(self.root_canvas, roi)
    }
}

/// 描画可能な物体を表す．
pub trait Drawable {
    /// この物体を描画するために必要な領域のサイズを返す．
    fn region_size(&self) -> Movement;

    /// 指定した位置にこの物体を描画する場合に必要となるROIを返す．
    fn get_roi(&self, left_top: Pos) -> RegionOfInterest {
        let size = self.region_size();
        RegionOfInterest::new(left_top, size)
    }

    /// この物体を指定したキャンバスに描画する．
    fn draw<C: Canvas>(&self, canvas: &mut C);

    /// 指定した位置にこの物体を描画する．
    fn draw_on_child<C: Canvas>(&self, left_top: Pos, parent_canvas: &mut C) {
        let roi = self.get_roi(left_top);
        let mut child_canvas = parent_canvas.child(roi);
        self.draw(&mut child_canvas);
    }
}

#[cfg(test)]
mod tests_root_canvas {
    use super::*;

    #[test]
    fn test_draw_cell() {
        let mut root_canvas = RootCanvas::new();

        let cell = {
            let c = SquareChar::new('a', 'a');
            let color = CanvasCellColor::new(Color::White, Color::Cyan);
            CanvasCell::new(c, color)
        };
        let pos = Pos::origin() + right(5) + below(3);

        // キャンバス内のあるセルを書き換え
        root_canvas.draw_cell(pos, cell);
        // ちゃんと書き換えられた?
        assert_eq!(cell, root_canvas.cells[3][5]);
    }

    #[test]
    fn test_clear() {
        let mut root_canvas = RootCanvas::new();

        let cell = {
            let c = SquareChar::new('a', 'a');
            let color = CanvasCellColor::new(Color::White, Color::Cyan);
            CanvasCell::new(c, color)
        };
        let pos = Pos::origin() + right(5) + below(3);

        // キャンバス内のあるセルを書き換え
        root_canvas.draw_cell(pos, cell);
        // キャンバスを初期状態に戻す
        root_canvas.clear();
        // ちゃんと戻った?
        assert_eq!(CanvasCell::default(), root_canvas.cells[3][5]);
    }

    #[test]
    fn test_child() {
        let mut root_canvas = RootCanvas::new();
        let pos = Pos::origin() + right(2) + below(3);
        let size = right(5) + below(6);
        let roi = RegionOfInterest::new(pos, size);

        let child = root_canvas.child(roi);

        assert_eq!(roi, child.roi);
    }
}

#[cfg(test)]
mod tests_child_canvas {
    use super::*;

    #[test]
    fn test_draw_cell() {
        let mut root_canvas = RootCanvas::new();

        // 子キャンバスを作る
        let pos = Pos::origin() + right(2) + below(3);
        let size = right(10) + below(10);
        let roi = RegionOfInterest::new(pos, size);

        let mut child = root_canvas.child(roi);
        let cell = {
            let c = SquareChar::new('a', 'a');
            let color = CanvasCellColor::new(Color::White, Color::Cyan);
            CanvasCell::new(c, color)
        };

        // 子キャンバスにおける描画位置を指定
        let pos = Pos::origin() + right(5) + below(3);
        // 描画
        child.draw_cell(pos, cell);
        // 親キャンバスのセルが書き換わっているはず
        assert_eq!(cell, root_canvas.cells[3 + 3][2 + 5]);
    }

    #[test]
    fn test_draw_cell_out_of_roi_right() {
        let mut root_canvas = RootCanvas::new();

        // 子キャンバスを作る
        let pos = Pos::origin() + right(2) + below(3);
        let size = right(5) + below(10);
        let roi = RegionOfInterest::new(pos, size);

        let mut child = root_canvas.child(roi);
        let cell = {
            let c = SquareChar::new('a', 'a');
            let color = CanvasCellColor::new(Color::White, Color::Cyan);
            CanvasCell::new(c, color)
        };

        // 子キャンバスにおける描画位置を指定．
        // しかしこれは子キャンバスのROI外
        let pos = Pos::origin() + right(5) + below(3);
        // 描画
        assert!(child.draw_cell(pos, cell).is_none());
        // 親キャンバスのセルは書き換わらないはず
        let cells1 = RootCanvas::new()
            .cells
            .iter()
            .flat_map(|row| row.to_vec())
            .collect::<Vec<_>>();
        let cells2 = root_canvas
            .cells
            .iter()
            .flat_map(|row| row.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(cells1, cells2);
    }

    #[test]
    fn test_draw_cell_out_of_roi_left() {
        let mut root_canvas = RootCanvas::new();

        // 子キャンバスを作る
        let pos = Pos::origin() + right(2) + below(3);
        let size = right(5) + below(10);
        let roi = RegionOfInterest::new(pos, size);

        let mut child = root_canvas.child(roi);
        let cell = {
            let c = SquareChar::new('a', 'a');
            let color = CanvasCellColor::new(Color::White, Color::Cyan);
            CanvasCell::new(c, color)
        };

        // 子キャンバスにおける描画位置を指定．
        // しかしこれは子キャンバスのROI外
        let pos = Pos::origin() + left(1) + below(2);
        // 描画
        assert!(child.draw_cell(pos, cell).is_none());
        // 親キャンバスのセルは書き換わらないはず
        let cells1 = RootCanvas::new()
            .cells
            .iter()
            .flat_map(|row| row.to_vec())
            .collect::<Vec<_>>();
        let cells2 = root_canvas
            .cells
            .iter()
            .flat_map(|row| row.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(cells1, cells2);
    }

    #[test]
    fn test_draw_cell_out_of_roi_below() {
        let mut root_canvas = RootCanvas::new();

        // 子キャンバスを作る
        let pos = Pos::origin() + right(2) + below(3);
        let size = right(5) + below(10);
        let roi = RegionOfInterest::new(pos, size);

        let mut child = root_canvas.child(roi);
        let cell = {
            let c = SquareChar::new('a', 'a');
            let color = CanvasCellColor::new(Color::White, Color::Cyan);
            CanvasCell::new(c, color)
        };

        // 子キャンバスにおける描画位置を指定．
        // しかしこれは子キャンバスのROI外
        let pos = Pos::origin() + right(4) + below(10);
        // 描画
        assert!(child.draw_cell(pos, cell).is_none());
        // 親キャンバスのセルは書き換わらないはず
        let cells1 = RootCanvas::new()
            .cells
            .iter()
            .flat_map(|row| row.to_vec())
            .collect::<Vec<_>>();
        let cells2 = root_canvas
            .cells
            .iter()
            .flat_map(|row| row.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(cells1, cells2);
    }

    #[test]
    fn test_draw_cell_out_of_roi_above() {
        let mut root_canvas = RootCanvas::new();

        // 子キャンバスを作る
        let pos = Pos::origin() + right(2) + below(3);
        let size = right(5) + below(10);
        let roi = RegionOfInterest::new(pos, size);

        let mut child = root_canvas.child(roi);
        let cell = {
            let c = SquareChar::new('a', 'a');
            let color = CanvasCellColor::new(Color::White, Color::Cyan);
            CanvasCell::new(c, color)
        };

        // 子キャンバスにおける描画位置を指定．
        // しかしこれは子キャンバスのROI外
        let pos = Pos::origin() + right(5) + above(1);
        // 描画
        assert!(child.draw_cell(pos, cell).is_none());
        // 親キャンバスのセルは書き換わらないはず
        let cells1 = RootCanvas::new()
            .cells
            .iter()
            .flat_map(|row| row.to_vec())
            .collect::<Vec<_>>();
        let cells2 = root_canvas
            .cells
            .iter()
            .flat_map(|row| row.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(cells1, cells2);
    }

    #[test]
    fn test_child_draw_cell() {
        let mut root_canvas = RootCanvas::new();

        // 子キャンバスを作る
        let pos = Pos::origin() + right(2) + below(3);
        let size = right(5) + below(10);
        let roi = RegionOfInterest::new(pos, size);

        let mut child = root_canvas.child(roi);

        // 孫キャンバスをつくる
        let pos = Pos::origin() + right(2) + below(3);
        let size = right(3) + below(4);
        let roi = RegionOfInterest::new(pos, size);

        let mut grandchild = child.child(roi);

        assert_eq!(PosX::right(2 + 2), grandchild.roi.left_top.x());
        assert_eq!(PosY::below(3 + 3), grandchild.roi.left_top.y());
        assert_eq!(right(3), grandchild.roi.size.x());
        assert_eq!(below(4), grandchild.roi.size.y());

        // 孫キャンバスに描画
        let cell = {
            let c = SquareChar::new('a', 'a');
            let color = CanvasCellColor::new(Color::White, Color::Cyan);
            CanvasCell::new(c, color)
        };

        grandchild.draw_cell(Pos::origin() + right(1) + below(1), cell);

        // 親キャンバスのセルが書き換わっているはず
        assert_eq!(cell, root_canvas.cells[3 + 3 + 1][2 + 2 + 1]);
    }
}
