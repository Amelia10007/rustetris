pub use console::Color;
use console::Style;

/// 表示用の文字を表す．
/// これは表示した際に，正方形領域内に描画されることを保証する．
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SquareChar(pub(super) [char; 2]);

impl SquareChar {
    /// # Panics
    /// 半角英数字以外の文字を指定した場合．
    pub fn new(left: char, right: char) -> SquareChar {
        debug_assert!(left.is_ascii() && !left.is_ascii_control());
        debug_assert!(right.is_ascii() && !right.is_ascii_control());
        Self([left, right])
    }
}

/// 表示する際の色を表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanvasCellColor {
    /// 前景色．
    foreground: Color,
    /// 背景色．
    background: Color,
}

impl CanvasCellColor {
    pub const fn new(foreground: Color, background: Color) -> CanvasCellColor {
        Self {
            foreground,
            background,
        }
    }

    /// 標準出力用でこの色を反映するためのスタイルを返す．
    pub(super) fn as_style(&self) -> Style {
        Style::default().fg(self.foreground).bg(self.background)
    }
}

impl Default for CanvasCellColor {
    fn default() -> Self {
        Self::new(Color::White, Color::Black)
    }
}

/// キャンバスの最小単位であるセルを表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanvasCell {
    /// このセルが表示する文字．
    pub c: SquareChar,
    /// このセルを表示するときの色．
    pub color: CanvasCellColor,
}

impl CanvasCell {
    pub const fn new(c: SquareChar, color: CanvasCellColor) -> CanvasCell {
        Self { c, color }
    }
}

impl Default for CanvasCell {
    fn default() -> Self {
        Self {
            c: SquareChar::new(' ', ' '),
            color: CanvasCellColor::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_char_new() {
        // コンストラクタが通るパターン
        let _c = SquareChar::new('a', 'a');
        let _c = SquareChar::new('a', '0');
        let _c = SquareChar::new('0', '0');
        let _c = SquareChar::new('0', ' ');
        let _c = SquareChar::new('0', '_');
    }

    #[test]
    #[should_panic]
    fn test_square_char_invalid_left() {
        let _c = SquareChar::new('\n', 'a');
    }

    #[test]
    #[should_panic]
    fn test_square_char_invalid_right() {
        let _c = SquareChar::new('a', '\n');
    }
}
