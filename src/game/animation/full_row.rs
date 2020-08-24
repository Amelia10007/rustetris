use super::*;
use crate::graphics::Canvas;

pub struct FullRow {
    field: AnimationField,
    filled_row_ys: Vec<PosY>,
    frame: AnimationFrame,
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

        // 揃ったラインが以前とまったく同一だった場合はアニメーションを表示しない．
        let filled_row_ys = if &filled_row_ys[..] == previous_filled_rows {
            vec![]
        } else {
            filled_row_ys
        };

        // 一ラインあたりの表示遷移フレーム*揃ったライン数+表示が遷移した後の追加表示フレーム数
        let max_frame_count = {
            let count_per_line = field.field.width() / 2;
            let additional_count = if filled_row_ys.is_empty() { 0 } else { 5 };
            count_per_line * filled_row_ys.len() + additional_count
        };
        let frame = AnimationFrame::with_frame_count(max_frame_count);

        Self {
            field,
            filled_row_ys,
            frame,
        }
    }
}

impl Animation for FullRow {
    type Finished = (AnimationField, Vec<PosY>);

    fn wait_next(self) -> AnimationResult<Self, Self::Finished> {
        match self.frame.wait_next() {
            Some(next_frame) => AnimationResult::InProgress(Self {
                frame: next_frame,
                ..self
            }),
            None => AnimationResult::Finished((self.field, self.filled_row_ys)),
        }
    }

    fn draw<C: Canvas>(&self, canvas: &mut C) {
        // まずは普通にフィールドを描画し，これにアニメーションを上書きしていく．
        self.field.draw(canvas);

        /*
        アニメーションは以下の流れで表示する．
        1. 揃ったラインのうち，最下段のラインについて左右端から横線を表示していく．
        2. ライン上の全セルに横線を表示したら，次のラインについて横線表示を同様にしていく．
        3. 一度横線表示が終わったラインについては，横線の代わりに合計何ライン揃ったのかラインの中心に描画する．
        */

        let count_per_line = self.field.field.width() / 2;
        let filled_row_count = self.frame.current_frame() / count_per_line;
        let filling_cell_count = self.frame.current_frame() % count_per_line;

        // 横線を表示し終えたラインたち
        for (i, &y) in (0..filled_row_count).zip(self.filled_row_ys.iter()) {
            // 合計何列揃ったのか描画
            let x = PosX::right(self.field.field.width() as i8 / 2);
            let pos = Pos(x, y);
            let colored_str = {
                let color = CanvasCellColor::new(Color::White, Color::Black);
                ColoredStr((i + 1).to_string(), color)
            };
            colored_str.draw_on_child(pos, canvas);
        }

        // 横線表示中のライン
        if let Some(&y) = self.filled_row_ys.get(filled_row_count) {
            for i in 0..filling_cell_count {
                let colored_str = {
                    let color = CanvasCellColor::new(Color::White, Color::Black);
                    ColoredStr("--", color)
                };
                // 左側
                let x = PosX::right(i as i8);
                let pos = Pos(x, y);
                colored_str.draw_on_child(pos, canvas);
                // 右側
                let x = PosX::right((self.field.field.width() - i - 1) as i8);
                let pos = Pos(x, y);
                colored_str.draw_on_child(pos, canvas);
            }
        }
    }
}
