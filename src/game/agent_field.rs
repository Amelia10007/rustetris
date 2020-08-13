use super::{Block, BlockSelector, Cell, Field};
use crate::data_type::Shake;
use crate::geometry::*;
use crate::graphics::*;

mod consts {
    /// Nextブロック列に格納されるブロックの数．
    pub const NEXT_BLOCK_NUM: usize = 2;
}

use consts::*;

/// ゲームにおけるブロック操作の結果を表す．
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationResult {
    /// 実際に操作ができて，フィールドや操作ブロックの状態が変化した．
    Done,
    ///操作は不可能だった．．フィールドや操作ブロックの状態は変化しなかった．
    Stay,
}

/// エージェントの操作対象となるフィールドを表す．
#[derive(Debug)]
pub struct AgentField {
    /// セルが配置されたフィールド．
    field: Field,
    /// 現在エージェントの操作対象となっているブロック．
    current_block: Block,
    current_block_pos: Pos,
    /// Nextブロックキュー．
    next_blocks: NextBlockQueue,
    /// Holdされているブロック．
    hold_block: Block,
}

impl AgentField {
    /// 指定したブロック生成器を用いてフィールドを初期化する．
    pub fn new<S: BlockSelector>(selector: &mut S) -> AgentField {
        let field = Field::empty();
        let current_block = selector.generate_block();
        let current_block_pos = find_block_appearance_pos(&field, &current_block).unwrap();
        let next_blocks = NextBlockQueue::fill(selector);
        let hold_block = selector.generate_block();

        Self {
            field,
            current_block,
            current_block_pos,
            next_blocks,
            hold_block,
        }
    }

    pub fn move_block_to_left(self) -> (Self, OperationResult) {
        let next_pos = self.current_block_pos + left(1);
        if is_arrangeable(&self.field, &self.current_block, next_pos) {
            let next_state = Self {
                current_block_pos: next_pos,
                ..self
            };
            (next_state, OperationResult::Done)
        } else {
            (self, OperationResult::Stay)
        }
    }

    pub fn move_block_to_right(self) -> (Self, OperationResult) {
        let next_pos = self.current_block_pos + right(1);
        if is_arrangeable(&self.field, &self.current_block, next_pos) {
            let next_state = Self {
                current_block_pos: next_pos,
                ..self
            };
            (next_state, OperationResult::Done)
        } else {
            (self, OperationResult::Stay)
        }
    }

    pub fn move_block_down(self) -> (Self, OperationResult) {
        let next_pos = self.current_block_pos + below(1);
        if is_arrangeable(&self.field, &self.current_block, next_pos) {
            let next_state = Self {
                current_block_pos: next_pos,
                ..self
            };
            (next_state, OperationResult::Done)
        } else {
            (self, OperationResult::Stay)
        }
    }

    pub fn drop_block(self) -> (Self, OperationResult) {
        let mut drop_shift = 0;

        loop {
            let next_pos = self.current_block_pos + below(drop_shift + 1);
            if is_arrangeable(&self.field, &self.current_block, next_pos) {
                drop_shift += 1;
            } else {
                break;
            }
        }

        let next_pos = self.current_block_pos + below(drop_shift);
        let next_state = Self {
            current_block_pos: next_pos,
            ..self
        };
        (next_state, OperationResult::Done)
    }

    pub fn rotate_block_clockwise(self) -> (Self, OperationResult) {
        let rotated_block = self.current_block.rotate_clockwise();
        let shift_max = self.current_block.cell_table_size() as i8 / 2;

        for y in Shake::<i8>::new().take_while(|y| y.abs() <= shift_max) {
            for x in Shake::<i8>::new().take_while(|x| x.abs() <= shift_max) {
                let shifted_pos = self.current_block_pos + right(x) + below(y);
                if is_arrangeable(&self.field, &rotated_block, shifted_pos) {
                    let next_state = Self {
                        current_block: rotated_block,
                        current_block_pos: shifted_pos,
                        ..self
                    };
                    return (next_state, OperationResult::Done);
                }
            }
        }

        (self, OperationResult::Stay)
    }

    pub fn rotate_block_unticlockwise(self) -> (Self, OperationResult) {
        let rotated_block = self.current_block.rotate_unticlockwise();
        let shift_max = self.current_block.cell_table_size() as i8 / 2;

        for y in Shake::<i8>::new().take_while(|y| y.abs() <= shift_max) {
            for x in Shake::<i8>::new().take_while(|x| x.abs() <= shift_max) {
                let shifted_pos = self.current_block_pos + right(x) + below(y);
                if is_arrangeable(&self.field, &rotated_block, shifted_pos) {
                    let next_state = Self {
                        current_block: rotated_block,
                        current_block_pos: shifted_pos,
                        ..self
                    };
                    return (next_state, OperationResult::Done);
                }
            }
        }

        (self, OperationResult::Stay)
    }

    pub fn hold_block(self) -> (Self, OperationResult) {
        let (current_block, hold_block) = (self.hold_block, self.current_block);

        match find_block_appearance_pos(&self.field, &current_block) {
            Some(pos) => (
                Self {
                    current_block,
                    current_block_pos: pos,
                    hold_block,
                    ..self
                },
                OperationResult::Done,
            ),
            None => (self, OperationResult::Stay),
        }
    }
}

impl Drawable for AgentField {
    fn region_size(&self) -> Movement {
        // フィールド用
        let field_region_size = self.field.region_size();
        // nextブロック用
        let block_region_size = self.next_blocks.into_iter().next().unwrap().region_size();
        // フィールドの右にnextブロック列とholdブロックを表示するので，
        let width = field_region_size.x() + right(1) + block_region_size.x();
        let height = field_region_size.y();

        width + height
    }

    fn draw<C: Canvas>(&self, canvas: &mut C) {
        let p = Pos::origin();
        // 左上にフィールドを描画
        self.field.draw_on_child(p, canvas);
        // 操作中のブロック描画
        self.current_block
            .draw_on_child(p + (self.current_block_pos - Pos::origin()), canvas);
        // フィールドから1マス開けて，右側に上から色々描画していく
        let p = p + self.field.region_size().x() + right(1);
        // Nextブロック列であることを示すテキスト
        let s = ColoredStr("Next", CanvasCellColor::new(Color::White, Color::Black));
        s.draw_on_child(p, canvas);
        let mut p = p + s.region_size().y();
        // nextブロック
        for next_block in self.next_blocks.into_iter() {
            let size = next_block.region_size();
            next_block.draw_on_child(p, canvas);
            p = p + size.y();
        }
        // Holdブロックであることを示すテキスト
        let s = ColoredStr("Hold", CanvasCellColor::new(Color::White, Color::Black));
        s.draw_on_child(p, canvas);
        let p = p + s.region_size().y();
        // Holdブロック
        self.hold_block.draw_on_child(p, canvas);
    }
}

/// Nextブロックキューを管理する．
#[derive(Debug, Clone)]
struct NextBlockQueue {
    /// Nextブロックキュー．
    blocks: [Block; NEXT_BLOCK_NUM],
}

impl NextBlockQueue {
    /// 満杯になったNextブロックキューを返す．
    pub fn fill<S: BlockSelector>(selector: &mut S) -> NextBlockQueue {
        let mut blocks = [Block::default(); NEXT_BLOCK_NUM];

        for block in blocks.iter_mut() {
            *block = selector.generate_block();
        }

        Self { blocks }
    }

    /// このキューからブロックを1つ取り出して返す．
    /// さらにキュー末尾に新しいブロックを追加し，キューが常に満杯になるようにする．
    pub fn pop_and_fill<S: BlockSelector>(&mut self, selector: &mut S) -> Block {
        let popped_block = self.blocks[0];

        for i in 0..self.blocks.len() - 1 {
            self.blocks[i] = self.blocks[i + 1];
        }

        self.blocks[self.blocks.len() - 1] = selector.generate_block();

        popped_block
    }
}

impl<'b> IntoIterator for &'b NextBlockQueue {
    type IntoIter = std::slice::Iter<'b, Block>;
    type Item = &'b Block;

    fn into_iter(self) -> Self::IntoIter {
        self.blocks.iter()
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
    for y in -shift_max..0 {
        for x in Shake::<i8>::new()
            .map(|x| x + field.width() as i8 / 2)
            .take(field.width())
        {
            let pos = Pos::origin() + below(y) + right(x);
            if is_arrangeable(field, block, pos) {
                return Some(pos);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
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

    mod tests_next_block_collection {
        use super::*;

        #[test]
        fn test_fill() {
            let queue = NextBlockQueue::fill(&mut block_generator());

            // キューに格納されたブロック列は，生成器が生成していくブロック列と同じになるはず
            let mut generator = block_generator();
            for &b in queue.into_iter() {
                assert_eq!(generator.generate_block(), b);
            }
        }

        #[test]
        fn test_pop_and_fill() {
            let mut generator = block_generator();
            let mut queue = NextBlockQueue::fill(&mut generator);
            // キューからブロック取り出し
            let popped1 = queue.pop_and_fill(&mut generator);
            let popped2 = queue.pop_and_fill(&mut generator);

            let mut generator = block_generator();

            // 生成器が最初に生成するブロックから順に，キューからブロックが取り出されていくはず
            assert_eq!(generator.generate_block(), popped1);
            assert_eq!(generator.generate_block(), popped2);

            // キューに格納されたブロック列は，生成器が生成していくブロック列と同じになるはず
            for &b in queue.into_iter() {
                assert_eq!(generator.generate_block(), b);
            }
        }
    }

    mod tests_function {
        use super::*;

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
}
