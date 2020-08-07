use crate::data_type::Pair;
use crate::geometry::Pos;
use ncurses::*;
use std::collections::HashMap;

mod consts {
    pub const CANVAS_WIDTH: usize = 80;
    pub const CANVAS_HEIHGT: usize = 40;
}

use consts::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanvasCell {
    c: char,
    color: CanvasCellColor,
}

impl Default for CanvasCell {
    fn default() -> Self {
        Self {
            c: char::default(),
            color: CanvasCellColor::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Black,
    White,
    Red,
    Yellow,
    Green,
    Blue,
    Cyan,
}

impl Color {
    fn to_ncurses_color(&self) -> i16 {
        match self {
            Color::Black => COLOR_BLACK,
            Color::White => COLOR_WHITE,
            Color::Red => COLOR_RED,
            Color::Yellow => COLOR_YELLOW,
            Color::Green => COLOR_GREEN,
            Color::Blue => COLOR_BLUE,
            Color::Cyan => COLOR_CYAN,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CanvasCellColor {
    foreground: Color,
    background: Color,
}

impl Default for CanvasCellColor {
    fn default() -> Self {
        Self {
            foreground: Color::White,
            background: Color::Black,
        }
    }
}

pub struct Canvas {
    cells: [[CanvasCell; CANVAS_WIDTH]; CANVAS_HEIHGT],
    color_pair_indexes: HashMap<CanvasCellColor, i16>,
    next_color_pair_index: i16,
}

impl Canvas {
    pub fn new() -> Canvas {
        Self {
            cells: [[CanvasCell::default(); CANVAS_WIDTH]; CANVAS_HEIHGT],
            color_pair_indexes: HashMap::new(),
            next_color_pair_index: 1,
        }
    }

    pub fn extract_region(&mut self, roi: RegionOfInterest) -> CanvasRegion<'_> {
        unimplemented!()
    }

    pub fn flush(&mut self) {
        for row in self.cells.iter() {
            for cell in row.iter() {
                let color = cell.color;
                let index = match self.color_pair_indexes.get(&color) {
                    Some(index) => *index,
                    None => {
                        self.color_pair_indexes
                            .insert(color, self.next_color_pair_index);
                        init_pair(
                            self.next_color_pair_index,
                            color.foreground.to_ncurses_color(),
                            color.background.to_ncurses_color(),
                        );
                        self.next_color_pair_index += 1;
                        self.next_color_pair_index - 1
                    }
                };
                attron(COLOR_PAIR(index));
                addstr(&cell.c.to_string());
            }
            addstr("\n");
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionOfInterest {
    pub left_top: Pair<isize>,
    pub size: Pair<isize>,
}

impl RegionOfInterest {
    pub const fn new(left_top: Pair<isize>, size: Pair<isize>) -> RegionOfInterest {
        Self { left_top, size }
    }
}

pub struct CanvasRegion<'c> {
    roi: RegionOfInterest,
    canvas: &'c mut Canvas,
}

impl<'c> CanvasRegion<'c> {
    pub fn draw_cell(&mut self, pos: Pos, cell: CanvasCell) {
        unimplemented!()
    }
}
