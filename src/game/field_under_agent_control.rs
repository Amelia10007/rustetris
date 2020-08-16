use super::{Block, BlockQueue, BlockSelector, Cell, Field};
use crate::data_type::Shake;
use crate::geometry::*;
use crate::graphics::*;
use crate::user::GameCommand;

/// ユーザが操作するブロックを表す．
#[derive(Debug, Clone)]
struct ControlledBlock {
    /// ブロック．
    block: Block,
    /// ブロックのセルテーブルのうち，最も左上のセルのフィールドにおける座標．
    left_top: Pos,
}

impl ControlledBlock {
    fn new(block: Block, left_top: Pos) -> ControlledBlock {
        Self { block, left_top }
    }

    /// このブロックの空でないセルとその位置を返す．
    fn iter_pos_and_occupied_cell(&self) -> impl IntoIterator<Item = (Pos, &'_ Cell)> + '_ {
        let diff = self.left_top - Pos::origin();
        self.block
            .iter_pos_and_occupied_cell()
            .map(move |(pos, cell)| (pos + diff, cell))
    }
}

/// `FieldUnderAgentControl`にゲーム操作を適用した結果を表す．
#[derive(Debug)]
pub enum GameCommandResult {
    /// 次の操作入力を待機してくれ．
    WaitNextCommand(FieldUnderAgentControl),
    /// ブロックの操作が確定した．次の処理に移行してくれ．
    /// このvariantはブロック設置後の`Field`と，今後のブロック操作に利用される`BlockQueue`をもつ．
    ProceedAnimation(Field, BlockQueue),
}

/// エージェントの操作対象となるフィールドを表す．
#[derive(Debug)]
pub struct FieldUnderAgentControl {
    /// セルが配置されたフィールド．
    field: Field,
    /// 現在エージェントの操作対象となっているブロック．
    controlled_block: ControlledBlock,
    block_queue: BlockQueue,
}

impl FieldUnderAgentControl {
    pub fn new<S: BlockSelector>(
        field: Field,
        mut block_queue: BlockQueue,
        selector: &mut S,
    ) -> Option<FieldUnderAgentControl> {
        // キューからブロックを取り出して操作ブロックとする
        let controlled_block = {
            let block = block_queue.pop_and_fill(selector);
            let pos = find_block_appearance_pos(&field, &block)?;
            ControlledBlock::new(block, pos)
        };

        Some(Self {
            field,
            controlled_block,
            block_queue,
        })
    }

    /// このフィールドに指定した操作を施した結果を返す．
    pub fn apply_command(mut self, command: GameCommand) -> GameCommandResult {
        use GameCommand::*;

        match command {
            // ブロック平行移動
            Right | Left | Down => {
                let shift: Movement = match command {
                    Right => right(1).into(),
                    Left => left(1).into(),
                    Down => below(1).into(),
                    _ => panic!("should not reach here"),
                };
                let next_pos = self.controlled_block.left_top + shift;
                if is_arrangeable(&self.field, &self.controlled_block.block, next_pos) {
                    let next_state = Self {
                        controlled_block: ControlledBlock::new(
                            self.controlled_block.block,
                            next_pos,
                        ),
                        ..self
                    };
                    GameCommandResult::WaitNextCommand(next_state)
                } else {
                    // 下移動ができなかった場合は次の状態へ移行
                    if command == Down {
                        let field = place_block(self.controlled_block, self.field);
                        GameCommandResult::ProceedAnimation(field, self.block_queue)
                    } else {
                        GameCommandResult::WaitNextCommand(self)
                    }
                }
            }
            // ブロックを真下に落とせるだけ落とす
            Drop => {
                // フィールドに収まり，かつフィールドの他セルと干渉しない範囲内でどこまで落とせるか計算
                let final_pos = {
                    let mut drop_shift = 0;
                    loop {
                        let next_pos = self.controlled_block.left_top + below(drop_shift + 1);
                        if is_arrangeable(&self.field, &self.controlled_block.block, next_pos) {
                            drop_shift += 1;
                        } else {
                            break self.controlled_block.left_top + below(drop_shift);
                        }
                    }
                };

                let dropped_block = ControlledBlock::new(self.controlled_block.block, final_pos);
                let field = place_block(dropped_block, self.field);
                // 次の状態へ移行
                GameCommandResult::ProceedAnimation(field, self.block_queue)
            }
            // ブロック回転
            RotateClockwise | RotateUnticlockwise => {
                let rotated_block = if command == RotateClockwise {
                    self.controlled_block.block.rotate_clockwise()
                } else {
                    self.controlled_block.block.rotate_unticlockwise()
                };
                let shift_max = self.controlled_block.block.cell_table_size() as i8 / 2;
                // 元の位置になるべく近くなるように，操作ブロックの位置を決めなおす．
                // これにより，操作ブロックの位置変更を許さない場合と比べてユーザにとってブロックが操作しやすくなる．
                for y in Shake::<i8>::new().take_while(|y| y.abs() <= shift_max) {
                    for x in Shake::<i8>::new().take_while(|x| x.abs() <= shift_max) {
                        let shifted_pos = self.controlled_block.left_top + right(x) + below(y);
                        if is_arrangeable(&self.field, &rotated_block, shifted_pos) {
                            let next_state = Self {
                                controlled_block: ControlledBlock::new(rotated_block, shifted_pos),
                                ..self
                            };
                            return GameCommandResult::WaitNextCommand(next_state);
                        }
                    }
                }

                GameCommandResult::WaitNextCommand(self)
            }
            // Holdブロック交換
            Hold => {
                let popped_block = self.block_queue.hold_block();
                // Holdブロックをフィールドに出現させられる場合のみ入れ替える
                match find_block_appearance_pos(&self.field, &popped_block) {
                    Some(pos) => {
                        self.block_queue
                            .swap_hold_block(self.controlled_block.block);
                        let next_state = Self {
                            controlled_block: ControlledBlock::new(popped_block, pos),
                            ..self
                        };
                        GameCommandResult::WaitNextCommand(next_state)
                    }
                    None => GameCommandResult::WaitNextCommand(self),
                }
            }
        }
    }
}

impl Drawable for FieldUnderAgentControl {
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
        // 操作中のブロック描画
        self.controlled_block
            .block
            .draw_on_child(p + (self.controlled_block.left_top - Pos::origin()), canvas);
        // フィールドから1マス開けて，右側にNextブロックやHoldブロックを描画していく
        let p = p + self.field.region_size().x() + right(1);
        self.block_queue.draw_on_child(p, canvas);
    }
}

/// 指定したブロックを指定した位置に配置可能かどうか返す．
/// ブロックの空でないセルとがすべてフィールド内に存在し，それらがフィールドの空でないセルが干渉しない場合に配置可能であると判定する．
pub fn is_arrangeable(field: &Field, block: &Block, block_left_top: Pos) -> bool {
    let diff = block_left_top - Pos::origin();
    block
        .iter_pos_and_occupied_cell()
        .map(|(pos, _cell)| pos + diff)
        .all(|pos| field.get(pos).map(|c| c.is_empty()).unwrap_or(false))
}

/// 指定したブロックを操作ブロックとしてフィールドに登場させる場合，その初期位置(ブロックセル群の左上の座標)を返す．
/// 初期位置は，そのブロックが配置可能な座標のうち，ブロックが可能な限りフィールド中央，フィールド上部に配置される位置となる．
/// # Returns
/// 指定したブロックが配置可能な場合，その左上座標`pos`を`Some(pos)`として返す．
/// 配置不可能な場合，`None`を返す．
pub fn find_block_appearance_pos(field: &Field, block: &Block) -> Option<Pos> {
    let shift_max = block.cell_table_size() as i8 / 2;
    for y in -shift_max..shift_max {
        for x in Shake::<i8>::new()
            .map(|x| x + field.width() as i8 / 2 - block.cell_table_size() as i8 / 2)
            .take(3)
        {
            let pos = Pos::origin() + below(y) + right(x);
            if is_arrangeable(field, block, pos) {
                return Some(pos);
            }
        }
    }
    None
}

/// 指定したブロックをフィールドに設置する．
/// ブロックの中にフィールドに収まらないセルが存在する場合，そのセルはフィールドに残らない．
/// # Panics on debug build
/// 1. 指定したブロックの空でないセルと，フィールドの空でないセルとが干渉していた場合．
fn place_block(controlled_block: ControlledBlock, mut field: Field) -> Field {
    for (pos, &cell) in controlled_block.iter_pos_and_occupied_cell() {
        if let Some(c) = field.get_mut(pos) {
            debug_assert!(c.is_empty());
            *c = cell;
        }
    }

    field
}

#[cfg(test)]
mod tests {
    use super::super::Cell;
    use super::super::QuadrupleBlockShape::*;
    use super::super::{BlockShape, BombTag};
    use super::*;

    struct QuadrupleBlockGenerator {
        current_index: usize,
    }

    impl BlockSelector for QuadrupleBlockGenerator {
        fn select_block_shape(&mut self) -> BlockShape {
            let shapes = [O, J, L, Z, S, T, I];

            let shape = shapes[self.current_index % shapes.len()];
            self.current_index += 1;
            shape.into()
        }

        fn select_bomb(&mut self, _: BlockShape) -> BombTag {
            BombTag::None
        }
    }

    fn block_generator() -> QuadrupleBlockGenerator {
        QuadrupleBlockGenerator { current_index: 0 }
    }

    #[test]
    fn test_is_arrangeable_empty_field() {
        let f = Field::empty();
        let b = block_generator().generate_block();
        let o = Pos::origin();
        // 左上ギリギリ
        assert!(is_arrangeable(&f, &b, o + left(2) + above(1)));
        // 上方向はみ出し
        assert!(!is_arrangeable(&f, &b, o + left(2) + above(2)));
        // 左方向はみ出し
        assert!(!is_arrangeable(&f, &b, o + left(3) + above(1)));
        // 右下ギリギリ
        assert!(is_arrangeable(&f, &b, o + right(6) + below(17)));
        // 下方向はみ出し
        assert!(!is_arrangeable(&f, &b, o + right(6) + below(18)));
        // 右方向はみ出し
        assert!(!is_arrangeable(&f, &b, o + right(7) + below(17)));
    }

    #[test]
    fn test_is_arrangeable_non_empty_field() {
        // 左上セルがすでに占有されているフィールド
        let f = {
            let mut field = Field::empty();
            *field.get_mut(Pos::origin()).unwrap() = Cell::Normal;
            field
        };
        let b = block_generator().generate_block();
        let o = Pos::origin();
        // 左上ギリギリに配置しようとすると，フィールドのセルと干渉するので配置できない
        assert!(!is_arrangeable(&f, &b, o + left(2) + above(1)));
        // 右や下方向に1だけずらせば配置可能
        assert!(is_arrangeable(&f, &b, o + left(1) + above(1)));
        assert!(is_arrangeable(&f, &b, o + left(2) + above(0)));
    }

    #[test]
    fn test_is_arrangeable_filled_field() {
        // 全セルがすでに占有されているフィールド
        let f = {
            let mut field = Field::empty();
            for y in 0..field.height() {
                for x in 0..field.width() {
                    let p = Pos::origin() + right(x as i8) + below(y as i8);
                    *field.get_mut(p).unwrap() = Cell::Normal;
                }
            }
            field
        };
        let b = block_generator().generate_block();
        let o = Pos::origin();
        // 左上ギリギリに配置しようとすると，フィールドのセルと干渉するので配置できない
        assert!(!is_arrangeable(&f, &b, o + left(2) + above(1)));
        // 右や下方向に1だけずらしても当然配置不可能
        assert!(!is_arrangeable(&f, &b, o + left(1) + above(1)));
        assert!(!is_arrangeable(&f, &b, o + left(2) + above(0)));
        // 右下ギリギリもだめ
        assert!(!is_arrangeable(&f, &b, o + right(6) + below(17)));
    }
}
