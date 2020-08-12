use super::{Block, BlockSelector, Cell, Field};
use crate::data_type::Shake;
use crate::geometry::*;
use crate::graphics::*;

mod consts {
    pub const NEXT_BLOCK_NUM: usize = 2;
}

use consts::*;

/// エージェントの操作対象となるフィールドを表す．
#[derive(Debug)]
pub struct AgentField {
    /// セルが配置されたフィールド．
    field: Field,
    /// 現在エージェントの操作対象となっているブロック．
    current_block: Block,
    current_block_pos: Pos,
    /// Nextブロックキュー．
    next_blocks: NextBlockCollection,
    /// Holdされているブロック．
    hold_block: Block,
}

impl AgentField {
    pub fn new<S: BlockSelector>(selector: &mut S) -> AgentField {
        let field = Field::empty();
        let current_block = selector.generate_block();
        let current_block_pos = find_block_appearance_pos(&field, &current_block).expect(&format!(
            "Cannot place a block on empty field. Block: {:?}",
            current_block
        ));
        let next_blocks = NextBlockCollection::fill(selector);
        let hold_block = selector.generate_block();

        Self {
            field,
            current_block,
            current_block_pos,
            next_blocks,
            hold_block,
        }
    }

    pub fn move_block_to_left(self) -> Self {
        let next_pos = self.current_block_pos + left(1);
        if is_arrangeable(&self.field, &self.current_block, next_pos) {
            Self {
                current_block_pos: next_pos,
                ..self
            }
        } else {
            self
        }
    }

    pub fn move_block_to_right(self) -> Self {
        let next_pos = self.current_block_pos + right(1);
        if is_arrangeable(&self.field, &self.current_block, next_pos) {
            Self {
                current_block_pos: next_pos,
                ..self
            }
        } else {
            self
        }
    }

    pub fn move_block_down(self) -> Self {
        let next_pos = self.current_block_pos + below(1);
        if is_arrangeable(&self.field, &self.current_block, next_pos) {
            Self {
                current_block_pos: next_pos,
                ..self
            }
        } else {
            self
        }
    }

    pub fn drop_block(self) -> Self {
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
        Self {
            current_block_pos: next_pos,
            ..self
        }
    }

    pub fn rotate_block_clockwise(self) -> Self {
        let rotated_block = self.current_block.rotate_clockwise();

        for y in Shake::<i8>::new().take_while(|y| y.abs() < 3) {
            for x in Shake::<i8>::new().take_while(|x| x.abs() < 3) {
                let shifted_pos = self.current_block_pos + right(x) + below(y);
                if is_arrangeable(&self.field, &rotated_block, shifted_pos) {
                    return Self {
                        current_block: rotated_block,
                        current_block_pos: shifted_pos,
                        ..self
                    };
                }
            }
        }

        self
    }

    pub fn rotate_block_unticlockwise(self) -> Self {
        let rotated_block = self.current_block.rotate_unticlockwise();

        for y in Shake::<i8>::new().take_while(|y| y.abs() < 3) {
            for x in Shake::<i8>::new().take_while(|x| x.abs() < 3) {
                let shifted_pos = self.current_block_pos + right(x) + below(y);
                if is_arrangeable(&self.field, &rotated_block, shifted_pos) {
                    return Self {
                        current_block: rotated_block,
                        current_block_pos: shifted_pos,
                        ..self
                    };
                }
            }
        }

        self
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
struct NextBlockCollection {
    /// Nextブロックキュー．
    blocks: [Block; NEXT_BLOCK_NUM],
}

impl NextBlockCollection {
    /// 満杯になったNextブロックキューを返す．
    pub fn fill<S: BlockSelector>(selector: &mut S) -> NextBlockCollection {
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

impl<'b> IntoIterator for &'b NextBlockCollection {
    type IntoIter = std::slice::Iter<'b, Block>;
    type Item = &'b Block;

    fn into_iter(self) -> Self::IntoIter {
        self.blocks.iter()
    }
}

pub fn is_arrangeable(field: &Field, block: &Block, block_left_top: Pos) -> bool {
    let diff = block_left_top - Pos::origin();
    // ブロックの空でないセルがすべてフィールド内に存在し，
    // そのセルがフィールド内の空でないセルと重ならない場合，そのブロックが配置可能であると判定する．
    block
        .iter_pos_and_occupied_cell()
        .map(|(pos, _cell)| pos + diff)
        .all(|pos| field.get(pos).map(|&c| c) == Some(Cell::Empty))
}

/// 指定したブロックを操作ブロックとしてフィールドに登場させる場合，その初期位置(ブロックセル群の左上の座標)を返す．
pub fn find_block_appearance_pos(field: &Field, block: &Block) -> Option<Pos> {
    for y in -3..0 {
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
