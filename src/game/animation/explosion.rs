use super::*;
use crate::data_type::Counter;
use crate::game::Cell;
use crate::graphics::Canvas;
use std::collections::HashSet;
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
    filled_row_count: usize,
    /// 爆発してはいないが，爆発に巻き込まれたボムセルの位置．
    /// 爆発の連鎖を表現するために利用される．
    caught_bomb_positions: HashSet<Pos>,
    /// 爆発に巻き込まれたセル(空，通常，ボムの全種類)の位置．
    /// 爆発アニメーションの描画に利用される．
    exploded_cell_positions: HashSet<Pos>,
    frame: AnimationFrame,
}

impl Explosion {
    pub fn try_init(
        field: AnimationField,
        filled_rows: &[PosY],
        current_chain: ChainCounter,
    ) -> ExplosionInitResult {
        let filled_row_count = filled_rows.len();
        let explosion_power = ExplosionPower::new(filled_row_count, &current_chain);

        let explosion_center_rows = field
            .field
            .rows()
            .filter(|row| filled_rows.contains(&row.y()));
        let explodable_center_cell_positions = explosion_center_rows
            .flat_map(|row| {
                row.cell_refs()
                    .into_iter()
                    .filter(|r| is_explodable(*r.cell()))
                    .map(|r| r.pos())
                    .collect::<Vec<_>>()
            })
            .collect::<HashSet<_>>();

        let exploded_cell_positions = scan_exploded_cell_positions(
            &field.field,
            &explodable_center_cell_positions,
            explosion_power,
        );
        let caught_bomb_positions = scan_caught_explosion_cell_positions(
            &field.field,
            &explodable_center_cell_positions,
            &exploded_cell_positions,
        );

        if exploded_cell_positions.is_empty() {
            ExplosionInitResult::Stay(field)
        } else {
            ExplosionInitResult::Explodes(Self {
                field,
                current_chain,
                filled_row_count,
                caught_bomb_positions,
                exploded_cell_positions,
                frame: animation_frame(),
            })
        }
    }
}

impl Animation for Explosion {
    type Finished = (AnimationField, ChainCounter);

    fn wait_next(mut self) -> AnimationResult<Self, Self::Finished> {
        // partial moveを防ぐためだけにclone()を使っている．他の方法を考えるのがベター．
        match self.frame.clone().wait_next() {
            Some(next_frame) => AnimationResult::InProgress(Self {
                frame: next_frame,
                ..self
            }),
            None => {
                // さっき爆発に巻き込まれた非爆心ボムセルがない場合，これ以上爆発は起きないので終了
                if self.caught_bomb_positions.is_empty() {
                    // 爆発に巻き込まれたセルは空セルになる
                    for &exploded_pos in self.exploded_cell_positions.iter() {
                        if let Some(c) = self.field.field.get_mut(exploded_pos) {
                            *c = Cell::Empty;
                        }
                    }
                    AnimationResult::Finished((self.field, self.current_chain.next()))
                } else {
                    // さっき爆発に巻き込まれた非爆心ボムセルがまだある場合
                    let explosion_power =
                        ExplosionPower::new(self.filled_row_count, &self.current_chain);
                    let explodable_center_cell_positions = &self.caught_bomb_positions;
                    let exploded_cell_positions = scan_exploded_cell_positions(
                        &self.field.field,
                        &explodable_center_cell_positions,
                        explosion_power,
                    );
                    let caught_bomb_positions = scan_caught_explosion_cell_positions(
                        &self.field.field,
                        &explodable_center_cell_positions,
                        &exploded_cell_positions,
                    );

                    // 爆発に巻き込まれたセルは空セルになる
                    for &exploded_pos in self.exploded_cell_positions.iter() {
                        if let Some(c) = self.field.field.get_mut(exploded_pos) {
                            *c = Cell::Empty;
                        }
                    }

                    let next_state = Self {
                        caught_bomb_positions,
                        exploded_cell_positions,
                        frame: animation_frame(),
                        ..self
                    };
                    AnimationResult::InProgress(next_state)
                }
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

#[derive(Debug, Clone, Copy)]
struct ExplosionPower {
    power: usize,
}

impl ExplosionPower {
    fn new(filled_row_count: usize, chain_counter: &ChainCounter) -> ExplosionPower {
        let power = filled_row_count + chain_counter.current_chain();
        Self { power }
    }
}

fn is_explodable(cell: Cell) -> bool {
    matches!(cell, Cell::Bomb | Cell::BigBombUpperLeft)
}

fn explosion_area(
    explosion_power: ExplosionPower,
    cell: Cell,
    pos: Pos,
) -> Option<RegionOfInterest> {
    use Cell::*;

    match cell {
        Bomb => Some(bomb_explosion_area(explosion_power, pos)),
        BigBombUpperLeft => Some(big_bomb_explosion_area(pos)),
        _ => None,
    }
}

fn bomb_explosion_area(explosion_power: ExplosionPower, pos: Pos) -> RegionOfInterest {
    let (x, y) = match explosion_power.power {
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

const fn animation_frame() -> AnimationFrame {
    AnimationFrame::with_frame_count(20)
}

fn scan_exploded_cell_positions(
    field: &Field,
    explodable_center_cell_positions: &HashSet<Pos>,
    explosion_power: ExplosionPower,
) -> HashSet<Pos> {
    explodable_center_cell_positions
        .iter()
        .filter_map(|&pos| explosion_area(explosion_power, *field.get(pos).unwrap(), pos))
        .flat_map(|roi| roi.iter_pos())
        .collect()
}

fn scan_caught_explosion_cell_positions(
    field: &Field,
    explodable_center_cell_positions: &HashSet<Pos>,
    exploded_cell_positions: &HashSet<Pos>,
) -> HashSet<Pos> {
    explodable_center_cell_positions
        .symmetric_difference(exploded_cell_positions)
        .into_iter()
        .filter(|&&pos| {
            field
                .get(pos)
                .map(|&cell| is_explodable(cell))
                .unwrap_or(false)
        })
        .map(|&pos| pos)
        .collect()
}
