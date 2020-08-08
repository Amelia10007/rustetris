use crate::geometry::*;
use crate::ncurses_wrapper::*;
use take_if::TakeIf;

mod consts {
    pub const CANVAS_WIDTH: usize = 80;
    pub const CANVAS_HEIHGT: usize = 24;
}

use consts::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanvasCell {
    c: char,
    color_pair: ColorPair,
}

impl Default for CanvasCell {
    fn default() -> Self {
        Self {
            c: char::default(),
            color_pair: ColorPair::default(),
        }
    }
}

pub struct Canvas {
    cells: [[CanvasCell; CANVAS_WIDTH]; CANVAS_HEIHGT],
}

impl Canvas {
    pub fn new() -> Canvas {
        Self {
            cells: [[CanvasCell::default(); CANVAS_WIDTH]; CANVAS_HEIHGT],
        }
    }

    pub fn sub_canvas(&mut self, roi: RegionOfInterest) -> SubCanvas<'_> {
        SubCanvas::new(self, roi)
    }

    pub fn write(&mut self, ncurses: &mut NcursesWrapper) -> Result<()> {
        for row in self.cells.iter() {
            for cell in row.iter() {
                let &CanvasCell { c, color_pair } = cell;
                ncurses.add_str(c.to_string(), color_pair)?;
            }
            ncurses.add_str("\n", ColorPair::default())?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionOfInterest {
    pub left_top: Pos,
    pub size: Movement,
}

impl RegionOfInterest {
    pub const fn new(left_top: Pos, size: Movement) -> RegionOfInterest {
        Self { left_top, size }
    }
}

pub struct SubCanvas<'c> {
    canvas: &'c mut Canvas,
    roi: RegionOfInterest,
}

impl<'c> SubCanvas<'c> {
    pub fn new(canvas: &'c mut Canvas, roi: RegionOfInterest) -> SubCanvas<'c> {
        Self { canvas, roi }
    }

    pub fn sub_canvas(&'c mut self, roi: RegionOfInterest) -> SubCanvas<'c> {
        let diff = roi.left_top - self.roi.left_top;
        let left_top = self.roi.left_top + diff;
        let roi = RegionOfInterest::new(left_top, roi.size);
        Self::new(self.canvas, roi)
    }

    pub fn write_cell(&mut self, pos: Pos, cell: CanvasCell) -> Option<()> {
        let RegionOfInterest { left_top, size } = self.roi;
        let diff = pos - left_top;
        let canvas_pos = left_top + diff;

        let x = canvas_pos
            .x()
            .as_positive_index()
            .take_if(|&x| x < size.x().as_positive_index().unwrap())?;
        let y = canvas_pos
            .y()
            .as_positive_index()
            .take_if(|&x| x < size.y().as_positive_index().unwrap())?;
        let c = self
            .canvas
            .cells
            .get_mut(y)
            .and_then(|row| row.get_mut(x))?;
        *c = cell;

        Some(())
    }
}

pub trait Drawable {
    fn region_size(&self) -> Movement;

    fn draw(&self, target: &mut SubCanvas<'_>);
}
