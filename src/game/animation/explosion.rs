use super::*;
use crate::data_type::Counter;
use crate::game::Cell;
use crate::graphics::Canvas;
use std::ops::RangeFrom;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainCounter(Counter<usize, RangeFrom<usize>>);

impl ChainCounter {
    pub fn new() -> ChainCounter {
        let counter = Counter::new(0..);
        Self(counter)
    }

    pub fn current_chain(&self) -> usize {
        self.0.current()
    }

    pub fn next(self) -> Self {
        Self(self.0.next())
    }
}

pub enum ExplosionInitResult {
    Explodes(Explosion),
    Stay(AnimationField),
}

pub struct Explosion {
    field: AnimationField,
    current_chain: ChainCounter,
    exploded_cell_positions: Vec<Pos>,
    frame: AnimationFrame,
}

impl Explosion {
    pub fn try_init(
        field: AnimationField,
        filled_rows: &[PosY],
        current_chain: ChainCounter,
    ) -> ExplosionInitResult {
        let filled_row_count = filled_rows.len();

        let mut exploded_cell_positions = vec![];
        let explosion_center_rows = field
            .field
            .rows()
            .filter(|row| filled_rows.contains(&row.y()));
        for row in explosion_center_rows {
            let explosion_center_cell_positions = row
                .cell_refs()
                .into_iter()
                .filter_map(|cell_ref| {
                    explosion_area(
                        filled_row_count,
                        &current_chain,
                        *cell_ref.cell(),
                        cell_ref.pos(),
                    )
                })
                .flat_map(|roi| roi.iter_pos());
            exploded_cell_positions.extend(explosion_center_cell_positions);
        }

        if exploded_cell_positions.is_empty() {
            ExplosionInitResult::Stay(field)
        } else {
            let frame = AnimationFrame::with_frame_count(20);
            ExplosionInitResult::Explodes(Self {
                field,
                current_chain,
                exploded_cell_positions,
                frame,
            })
        }
    }
}

impl Animation for Explosion {
    type Finished = (AnimationField, ChainCounter);

    fn wait_next(mut self) -> AnimationResult<Self, Self::Finished> {
        match self.frame.wait_next() {
            Some(next_frame) => AnimationResult::InProgress(Self {
                frame: next_frame,
                ..self
            }),
            None => {
                // 爆発に巻き込まれたセルは空セルになる
                for exploded_pos in self.exploded_cell_positions.into_iter() {
                    if let Some(c) = self.field.field.get_mut(exploded_pos) {
                        *c = Cell::Empty;
                    }
                }
                AnimationResult::Finished((self.field, self.current_chain.next()))
            }
        }
    }

    fn draw<C: Canvas>(&self, canvas: &mut C) {
        let explosion_cell = {
            use Color::*;
            let color = CanvasCellColor::new(Yellow, Black);
            let c = if self.frame.current_frame() % 2 == 0 {
                'x'
            } else {
                '+'
            };
            CanvasCell::new(SquareChar::new(c, c), color)
        };

        self.field.draw(canvas);

        for &pos in self.exploded_cell_positions.iter() {
            canvas.draw_cell(pos, explosion_cell);
        }
    }
}

fn explosion_area(
    filled_row_count: usize,
    chain_counter: &ChainCounter,
    cell: Cell,
    pos: Pos,
) -> Option<RegionOfInterest> {
    use Cell::*;

    match cell {
        Bomb => Some(bomb_explosion_area(filled_row_count, chain_counter, pos)),
        BigBombUpperLeft => Some(big_bomb_explosion_area(pos)),
        _ => None,
    }
}

fn bomb_explosion_area(
    filled_row_count: usize,
    chain_counter: &ChainCounter,
    pos: Pos,
) -> RegionOfInterest {
    let explosion_strongness = filled_row_count + chain_counter.current_chain();
    debug_assert!(explosion_strongness > 0);

    let (x, y) = match explosion_strongness {
        1 => (3, 0),
        2 => (3, 1),
        3 => (3, 2),
        4 => (3, 3),
        5 | 6 => (4, 4),
        7 | 8 => (5, 5),
        9 | 10 => (6, 6),
        11 | 12 => (7, 7),
        _ => (8, 8),
    };

    let left_top = pos + left(x) + above(y);
    let size = Movement(right(x * 2 + 1), below(y * 2 + 1));

    RegionOfInterest::new(left_top, size)
}

fn big_bomb_explosion_area(big_bomb_upper_left_pos: Pos) -> RegionOfInterest {
    let left_top = big_bomb_upper_left_pos + left(4) + above(4);
    let size = Movement(right(10), below(10));
    RegionOfInterest::new(left_top, size)
}
