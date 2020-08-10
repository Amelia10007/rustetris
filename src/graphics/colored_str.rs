use super::*;
use crate::geometry::*;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColoredStr<S>(pub S, pub CanvasCellColor);

impl<S: AsRef<str>> Drawable for ColoredStr<S> {
    fn region_size(&self) -> Movement {
        let square_char_len = (self.0.as_ref().len() + 1) / 2;
        right(square_char_len as i8) + below(1)
    }

    fn draw<C: Canvas>(&self, canvas: &mut C) {
        self.0
            .as_ref()
            .chars()
            .chunks(2)
            .into_iter()
            .map(|mut chunk| {
                let left = chunk.next().unwrap();
                let right = chunk.next().unwrap_or(' ');
                SquareChar([left, right])
            })
            .map(|c| CanvasCell::new(c, self.1))
            .enumerate()
            .for_each(|(i, cell)| {
                let pos = Pos::origin() + right(i as i8);
                canvas.draw_cell(pos, cell);
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_size() {
        let even_len_str = ColoredStr("MinatoAqua", CanvasCellColor::default());
        let size = even_len_str.region_size();

        assert_eq!(right(5) + below(1), size);

        let odd_len_str = ColoredStr("Hello", CanvasCellColor::default());
        let size = odd_len_str.region_size();

        assert_eq!(right(3) + below(1), size);
    }
}
