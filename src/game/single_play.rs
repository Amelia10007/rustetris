use super::field_animation::{Animation, AnimationField, Drawer, PlaceBlock};
use super::field_under_agent_control::FieldUnderAgentControl;
use super::{BlockQueue, BlockSelector, BlockShape, BombTag, Field};
use crate::graphics::*;
use crate::user::GameCommand;

struct QuadrupleBlockGenerator {
    current_index: usize,
}

impl QuadrupleBlockGenerator {
    fn new() -> QuadrupleBlockGenerator {
        Self { current_index: 0 }
    }
}

impl BlockSelector for QuadrupleBlockGenerator {
    fn select_block_shape(&mut self) -> BlockShape {
        use super::QuadrupleBlockShape::*;

        let shapes = [O, J, L, Z, S, T, I];

        let shape = shapes[self.current_index % shapes.len()];
        self.current_index = (self.current_index + 1) % shapes.len();
        shape.into()
    }

    fn select_bomb(&mut self, _: BlockShape) -> BombTag {
        BombTag::Single(0)
    }
}

/// 一人プレイエンドレスゲームを実行する．
pub fn execute_game<I, D>(input: I, drawer: &mut D)
where
    I: Fn() -> GameCommand,
    D: Drawer,
{
    let mut block_generator = QuadrupleBlockGenerator::new();

    let mut field = Field::empty();
    let mut block_queue = BlockQueue::new(&mut block_generator);

    loop {
        let mut agent_field =
            match FieldUnderAgentControl::new(field, block_queue, &mut block_generator) {
                Some(field) => field,
                // ブロックをもう置けなくなったらゲーム終了
                None => break,
            };
        // 最初の状態を描画
        drawer.clear();
        agent_field.draw(drawer.canvas_mut());
        drawer.show();

        // ブロックの設置位置が確定するまでユーザからの入力を受け付ける
        let (confirmed_field, confirmed_block_queue) = loop {
            use super::field_under_agent_control::GameCommandResult::*;

            match agent_field.apply_command(input()) {
                WaitNextCommand(next_field) => agent_field = next_field,
                ProceedAnimation(field, block_queue) => break (field, block_queue),
            }
            drawer.clear();
            agent_field.draw(drawer.canvas_mut());
            drawer.show();
        };

        let animation_field = AnimationField::new(confirmed_field, confirmed_block_queue);
        let place_block_animation = PlaceBlock::new(animation_field);
        // アニメーション実行
        let finished_animation_field = place_block_animation.execute(drawer);

        // 次の操作のためにフィールドとキューを更新
        field = finished_animation_field.field;
        block_queue = finished_animation_field.block_queue;
    }
}
