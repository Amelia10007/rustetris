use super::*;
use crate::data_type::Counter;
use crate::graphics::Canvas;
use std::ops::Range;

pub struct FullRow {
    field: AnimationField,
    filled_rows: Vec<PosY>,
    counters: Vec<Counter<usize, Range<usize>>>,
}

impl FullRow {
    pub fn new(field: AnimationField, previous_filled_rows: &[PosY]) -> FullRow {
        let filled_row_ys = field
            .field
            .rows()
            .enumerate()
            .map(|(y, row)| (PosY::below(y as i8), row))
            .filter(|(_y, row)| row.iter().all(|cell| !cell.is_empty()))
            .map(|(y, _row)| y)
            .collect::<Vec<_>>();

        let ys = if &filled_row_ys[..] == previous_filled_rows {
            vec![]
        } else {
            filled_row_ys
        };

        let counter = Counter::new(0..field.field.width() / 2);
        let counters = vec![counter; ys.len()];

        Self {
            field,
            filled_rows: ys,
            counters,
        }
    }
}

impl Animation for FullRow {
    type Finished = (AnimationField, Vec<PosY>);

    fn wait_next(mut self) -> AnimationResult<Self, Self::Finished> {
        AnimationFrame::with_frame_count(1).wait_next();
        if self.filled_rows.is_empty() || self.counters.iter().all(|c| c.is_ended()) {
            AnimationResult::Finished((self.field, self.filled_rows))
        } else {
            // 先頭のカウントを増やす
            self.counters[0].next();
            // 上段の揃ったラインの描画か終わったらこのラインの描画も開始
            for i in 1..self.counters.len() {
                let previous = &self.counters[i - 1];
                if self.counters[i - 1].is_ended() {
                    self.counters[i].next();
                }
            }

            AnimationResult::InProgress(self)
        }
    }

    fn draw<C: Canvas>(&self, canvas: &mut C) {
        self.field.draw(canvas);

        for (i, (counter, &y)) in self
            .counters
            .iter()
            .zip(self.filled_rows.iter())
            .enumerate()
        {
            // 揃ったところに横線描画
            if !counter.is_ended() {
                for i in 0..counter.current() {
                    // 左側
                    let x = PosX::right(i as i8);
                    let pos = Pos(x, y);
                    let colored_str = {
                        let color = CanvasCellColor::new(Color::White, Color::Black);
                        ColoredStr("--", color)
                    };
                    colored_str.draw_on_child(pos, canvas);
                    // 右側
                    let x = PosX::right((self.field.field.width() - i - 1) as i8);
                    let pos = Pos(x, y);
                    colored_str.draw_on_child(pos, canvas);
                }
            }
            // 揃ったラインの中央に，合計何列揃ったのか描画
            if counter.is_ended() {
                let x = PosX::right(self.field.field.width() as i8 / 2);
                let pos = Pos(x, y);
                let colored_str = {
                    let color = CanvasCellColor::new(Color::White, Color::Black);
                    ColoredStr(i.to_string(), color)
                };
                colored_str.draw_on_child(pos, canvas);
            }
        }
    }
}
