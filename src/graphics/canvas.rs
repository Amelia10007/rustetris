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
    fn draw_cell(&mut self, pos: Pos, cell: CanvasCell);

    /// このキャンバスから指定した注目領域を切り抜き，子キャンバスとして返す．
    /// このキャンバスと子キャンバスは状態を共有する．
    /// 子キャンバス上のセルを描画すると，それに対応したこのキャンバスのセルも変更される．
    fn child(&mut self, roi: RegionOfInterest) -> ChildCanvas<'_>;
}

/// 描画可能な物体を表す．
pub trait Drawable {
    /// この物体を描画するために必要な領域のサイズを返す．
    fn region_size(&self) -> Movement;

    /// この物体を指定したキャンバスに描画する．
    fn draw<C: Canvas>(&self, canvas: &mut C);
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

    /// 標準出力にこのキャンバスのないようを表示するための文字列を生成する．
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
    fn draw_cell(&mut self, pos: Pos, cell: CanvasCell) {
        if let Some(y) = pos.y().as_positive_index() {
            if let Some(x) = pos.x().as_positive_index() {
                if let Some(c) = self.cells.get_mut(y).and_then(|row| row.get_mut(x)) {
                    *c = cell;
                }
            }
        }
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
    fn draw_cell(&mut self, pos: Pos, cell: CanvasCell) {
        let diff = pos - Pos::origin();
        let root_canvas_pos = self.roi.left_top + diff;

        self.root_canvas.draw_cell(root_canvas_pos, cell)
    }

    fn child(&mut self, roi: RegionOfInterest) -> ChildCanvas<'_> {
        let diff = roi.left_top - Pos::origin();
        let left_top = self.roi.left_top + diff;
        let roi = RegionOfInterest::new(left_top, roi.size);
        ChildCanvas::new(self.root_canvas, roi)
    }
}
