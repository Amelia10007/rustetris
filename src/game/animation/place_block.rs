use super::*;
use crate::graphics::Canvas;

/// ブロックを設置するときのアニメーション．
pub struct PlaceBlock {
    field: AnimationField,
    frame: AnimationFrame,
}

impl PlaceBlock {
    pub fn new(field: AnimationField) -> PlaceBlock {
        Self {
            field,
            frame: AnimationFrame::with_frame_count(5),
        }
    }
}

impl Animation for PlaceBlock {
    type Finished = AnimationField;

    fn wait_next(self) -> AnimationResult<Self, Self::Finished> {
        match self.frame.wait_next() {
            Some(next_frame) => AnimationResult::InProgress(Self {
                field: self.field,
                frame: next_frame,
            }),
            None => AnimationResult::Finished(self.field),
        }
    }

    fn draw<C: Canvas>(&self, canvas: &mut C) {
        self.field.draw(canvas);
    }
}
