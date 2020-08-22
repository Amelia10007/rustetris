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

    pub fn next(&mut self) {
        self.0.next().expect("chain must not have upper limit");
    }
}

pub struct Explosion {
    field: AnimationField,
    exploded_cell_positions: Vec<Pos>,
}

impl Explosion {
    pub fn new(
        field: AnimationField,
        filled_rows: Vec<PosY>,
        current_chain: ChainCounter,
    ) -> Explosion {
        let filled_row_count = filled_rows.len();
        let exploded_cell_positions = field
            .field
            .rows()
            .enumerate()
            .flat_map(move |(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, &cell)| (Pos::origin() + right(x as i8) + below(y as i8), cell))
                    .collect::<Vec<_>>()
            })
            .filter_map(|(pos, cell)| explosion_area(filled_row_count, &current_chain, cell, pos))
            .flat_map(|roi| roi.iter_pos())
            .collect::<Vec<_>>();

        Self {
            field,
            exploded_cell_positions,
        }
    }
}

impl Animation for Explosion {
    type Finished = AnimationField;

    fn wait_next(mut self) -> AnimationResult<Self, Self::Finished> {
        unimplemented!()
    }

    fn draw<C: Canvas>(&self, canvas: &mut C) {
        self.field.draw(canvas);
        unimplemented!()
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
