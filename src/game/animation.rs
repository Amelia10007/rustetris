mod drop_cell;
mod explosion;
mod full_row;
mod place_block;

use super::{BlockQueue, Field};
use crate::geometry::*;
use crate::graphics::*;
pub use drop_cell::DropCell;
pub use explosion::{ChainCounter, Explosion, ExplosionInitResult};
pub use full_row::FullRow;
pub use place_block::PlaceBlock;

/// アニメーション表示用のフィールドを表す．
pub struct AnimationField {
    pub field: Field,
    pub block_queue: BlockQueue,
}

impl AnimationField {
    pub fn new(field: Field, block_queue: BlockQueue) -> AnimationField {
        Self { field, block_queue }
    }
}

impl Drawable for AnimationField {
    fn region_size(&self) -> Movement {
        use std::cmp::max;

        // フィールド用
        let field_region_size = self.field.region_size();
        // nextブロック用
        let queue_region_size = self.block_queue.region_size();
        // フィールドの右にnextブロック列とholdブロックを表示するので，
        let width = field_region_size.x() + right(1) + queue_region_size.x();
        let height = max(field_region_size.y(), queue_region_size.y());

        width + height
    }

    fn draw<C: Canvas>(&self, canvas: &mut C) {
        let p = Pos::origin();
        // 左上にフィールドを描画
        self.field.draw_on_child(p, canvas);
        // フィールドから1マス開けて，右側にNextブロックやHoldブロックを描画していく
        let p = p + self.field.region_size().x() + right(1);
        self.block_queue.draw_on_child(p, canvas);
    }
}

/// アニメーションにおけるフレームを表し，アニメーションの遷移や終了タイミングを制御する．
#[derive(Clone)]
pub struct AnimationFrame {
    /// 現在のフレームカウント．
    current: usize,
    /// フレームカウントがこれに達したらアニメーションを終了する．
    end: usize,
}

impl AnimationFrame {
    /// アニメーションが終了するまでのフレーム数を指定する．
    pub const fn with_frame_count(end: usize) -> AnimationFrame {
        Self { current: 0, end }
    }

    /// 現在の経過フレーム数を返す．
    pub const fn current_frame(&self) -> usize {
        self.current
    }

    /// アニメーションの終了時のフレーム数を返す．
    pub const fn end_frame(&self) -> usize {
        self.end
    }

    /// 次のアニメーション遷移タイミングまで処理を中断する．
    /// # Returns
    /// アニメーションが終了する場合は`None`を返す．
    /// アニメーションがまだ終了しない場合は，次のフレーム`frame`を`Some(frame)`として返す．
    pub fn wait_next(self) -> Option<AnimationFrame> {
        debug_assert!(self.current <= self.end);
        if self.current == self.end {
            None
        } else {
            std::thread::sleep(std::time::Duration::from_millis(50));
            let next = Self {
                current: self.current + 1,
                end: self.end,
            };
            Some(next)
        }
    }
}

/// アニメーション描画のための機能を示すトレイト．
pub trait Drawer {
    /// 描画対象となるキャンバス．
    type Canvas: Canvas;

    /// 描画対象となるキャンバスを返す．
    fn canvas_mut(&mut self) -> &mut Self::Canvas;

    /// 現在の表示内容をすべて消去する．
    fn clear(&mut self);

    /// 現在の描画内容を反映する．
    fn show(&mut self);
}

pub enum AnimationResult<P, F> {
    InProgress(P),
    Finished(F),
}

/// アニメーションを表すトレイト．
pub trait Animation: Sized {
    /// アニメーション終了時に出力される型．
    type Finished;

    /// 次のアニメーション遷移タイミングまで待機する．
    /// # Returns
    /// アニメーションが終了する場合は`AnimationResult::Finished(...)`を返す．
    /// アニメーションがまだ終了しない場合は，次のアニメーションを表す`AnimationResult::InProgress(...)`を返す．
    fn wait_next(self) -> AnimationResult<Self, Self::Finished>;

    /// 現在のアニメーションを描画する．
    fn draw<C: Canvas>(&self, canvas: &mut C);

    /// 指定したアニメーション表示機能に対して，このアニメーションが終了するまで繰り返しコンテンツを表示する．
    fn execute<D: Drawer>(mut self, drawer: &mut D) -> Self::Finished {
        // 最初の状態を描画
        drawer.clear();
        self.draw(drawer.canvas_mut());
        drawer.show();

        loop {
            match self.wait_next() {
                AnimationResult::InProgress(next) => self = next,
                AnimationResult::Finished(f) => break f,
            }
            drawer.clear();
            self.draw(drawer.canvas_mut());
            drawer.show();
        }
    }
}
