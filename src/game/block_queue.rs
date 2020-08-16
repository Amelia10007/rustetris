use super::{Block, BlockSelector};
use crate::geometry::*;
use crate::graphics::*;

mod consts {
    /// Nextブロック列に格納されるブロックの数．
    pub const NEXT_BLOCK_NUM: usize = 2;
}

use consts::*;

/// Nextブロックキューを管理する．
#[derive(Debug, Clone)]
struct NextBlockQueue {
    /// Nextブロックキュー．
    blocks: [Block; NEXT_BLOCK_NUM],
}

impl NextBlockQueue {
    /// 満杯になったNextブロックキューを返す．
    fn fill<S: BlockSelector>(selector: &mut S) -> NextBlockQueue {
        let mut blocks = [Block::default(); NEXT_BLOCK_NUM];

        for block in blocks.iter_mut() {
            *block = selector.generate_block();
        }

        Self { blocks }
    }

    /// このキューからブロックを1つ取り出して返す．
    /// さらにキュー末尾に新しいブロックを追加し，キューが常に満杯になるようにする．
    fn pop_and_fill<S: BlockSelector>(&mut self, selector: &mut S) -> Block {
        let popped_block = self.blocks[0];

        for i in 0..self.blocks.len() - 1 {
            self.blocks[i] = self.blocks[i + 1];
        }

        self.blocks[self.blocks.len() - 1] = selector.generate_block();

        popped_block
    }
}

/// NextブロックおよびHoldブロックを管理する．
#[derive(Debug)]
pub struct BlockQueue {
    /// Nextブロック．
    next_blocks: NextBlockQueue,
    /// Holdブロック．
    hold_block: Block,
}

impl BlockQueue {
    pub fn new<S: BlockSelector>(selector: &mut S) -> BlockQueue {
        let next_blocks = NextBlockQueue::fill(selector);
        let hold_block = selector.generate_block();
        Self {
            next_blocks,
            hold_block,
        }
    }

    /// Nextブロックキューからひとつブロックを取り出す．
    /// Nextブロックキューには新たなブロックが追加される．
    pub fn pop_and_fill<S: BlockSelector>(&mut self, selector: &mut S) -> Block {
        self.next_blocks.pop_and_fill(selector)
    }

    /// 現在のHoldブロックを返す．
    pub fn hold_block(&self) -> Block {
        self.hold_block
    }

    /// 指定したブロックと現在のHoldブロックを入れ替える．
    pub fn swap_hold_block(&mut self, mut block: Block) -> Block {
        std::mem::swap(&mut block, &mut self.hold_block);
        block
    }
}

impl Drawable for BlockQueue {
    fn region_size(&self) -> Movement {
        // ブロック用
        let block_region_size = self.next_blocks.blocks.iter().next().unwrap().region_size();
        // フィールドの右にnextブロック列とholdブロックを表示するので，
        let width = block_region_size.x();
        let y = block_region_size.y();
        // テキスト表示，Nextブロック2つ，テキスト表示，Holdブロック表示
        let height = below(1) + y + y + below(1) + y;

        width + height
    }

    fn draw<C: Canvas>(&self, canvas: &mut C) {
        let p = Pos::origin();
        // Nextブロック列であることを示すテキスト
        let s = ColoredStr("Next", CanvasCellColor::new(Color::White, Color::Black));
        s.draw_on_child(p, canvas);
        let mut p = p + s.region_size().y();
        // nextブロック
        for next_block in self.next_blocks.blocks.iter() {
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

    #[test]
    fn test_fill() {
        let queue = NextBlockQueue::fill(&mut block_generator());

        // キューに格納されたブロック列は，生成器が生成していくブロック列と同じになるはず
        let mut generator = block_generator();
        for &b in queue.blocks.iter() {
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
        for &b in queue.blocks.iter() {
            assert_eq!(generator.generate_block(), b);
        }
    }
}
