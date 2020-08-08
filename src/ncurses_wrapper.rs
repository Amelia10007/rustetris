use self::NcursesError::*;
use ncurses::*;
use std::collections::{HashMap, HashSet};

pub type Result<T> = std::result::Result<T, NcursesError>;

#[derive(Debug)]
pub enum NcursesError {
    Initialization,
    Api(String),
}

macro_rules! call_ncurses {
    ($x:expr) => {
        match $x {
            ncurses::OK => Ok(()),
            code => Err(Api(format!(
                "ncurses api call '{}' failed with code {}.",
                stringify!($x),
                code
            ))),
        }
    };
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
pub struct ColorPair {
    foreground: Color,
    background: Color,
}

impl ColorPair {
    pub const fn new(foreground: Color, background: Color) -> ColorPair {
        Self {
            foreground,
            background,
        }
    }
}

impl Default for ColorPair {
    fn default() -> Self {
        Self::new(Color::White, Color::Black)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Key(u8);

impl Default for Key {
    fn default() -> Self {
        Key(0)
    }
}

pub struct NcursesWrapper {
    color_pair_indexes: HashMap<ColorPair, i16>,
    next_color_pair_index: i16,
}

impl NcursesWrapper {
    pub fn new() -> Result<NcursesWrapper> {
        if initscr().is_null() {
            return Err(Initialization);
        }

        // non-blocking input
        timeout(0);
        // do not display typed words
        call_ncurses!(noecho())?;
        // enable color
        call_ncurses!(start_color())?;

        Ok(Self {
            color_pair_indexes: HashMap::new(),
            next_color_pair_index: 1,
        })
    }

    pub fn erase(&mut self) -> Result<()> {
        call_ncurses!(erase())
    }

    pub fn refresh(&mut self) -> Result<()> {
        call_ncurses!(refresh())
    }

    pub fn add_str<S: AsRef<str>>(&mut self, content: S, color_pair: ColorPair) -> Result<()> {
        let index = match self.color_pair_indexes.get(&color_pair) {
            Some(index) => *index,
            None => {
                self.color_pair_indexes
                    .insert(color_pair, self.next_color_pair_index);
                call_ncurses!(init_pair(
                    self.next_color_pair_index,
                    color_pair.foreground.to_ncurses_color(),
                    color_pair.background.to_ncurses_color(),
                ))?;
                self.next_color_pair_index += 1;
                self.next_color_pair_index - 1
            }
        };
        call_ncurses!(attron(COLOR_PAIR(index)))?;
        call_ncurses!(addstr(content.as_ref()))
    }

    pub fn keys(&mut self) -> HashSet<Key> {
        const NO_INPUT: i32 = -1;

        let mut keys = HashSet::new();

        loop {
            match getch() {
                NO_INPUT => break,
                c => {
                    keys.insert(Key(c as u8));
                }
            }
        }
        keys
    }
}

impl Drop for NcursesWrapper {
    fn drop(&mut self) {
        endwin();
    }
}
