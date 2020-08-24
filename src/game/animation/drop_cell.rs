use super::*;
use crate::graphics::Canvas;
use itertools::Itertools;
use std::collections::HashSet;
use take_if::TakeIf;

pub struct DropCell {
    field: AnimationField,
    floating_cell_positions: HashSet<Pos>,
}

impl DropCell {
    pub fn new(field: AnimationField) -> DropCell {
        let floating_cell_positions = scan_floating_cell_positions(&field.field);
        Self {
            field,
            floating_cell_positions,
        }
    }
}

impl Animation for DropCell {
    type Finished = AnimationField;

    fn wait_next(mut self) -> AnimationResult<Self, Self::Finished> {
        AnimationFrame::with_frame_count(1).wait_next();

        if self.floating_cell_positions.is_empty() {
            AnimationResult::Finished(self.field)
        } else {
            // 下のラインにあるセルから落としていく
            for pos in self
                .floating_cell_positions
                .into_iter()
                .sorted_by_key(|pos| pos.y())
                .rev()
            {
                use crate::game::Cell;

                let destination = pos + below(1);
                // 移動対象のセルは空でないはず
                debug_assert!(matches!(
                    self.field.field.get(pos).map(|c| c.is_empty()),
                    Some(false)
                ));
                // 移動
                *self.field.field.get_mut(destination).unwrap() =
                    *self.field.field.get(pos).unwrap();
                *self.field.field.get_mut(pos).unwrap() = Cell::Empty;
            }

            let floating_cell_positions = scan_floating_cell_positions(&self.field.field);

            AnimationResult::InProgress(Self {
                floating_cell_positions,
                ..self
            })
        }
    }

    fn draw<C: Canvas>(&self, canvas: &mut C) {
        self.field.draw(canvas);
    }
}

fn scan_floating_cell_positions(field: &Field) -> HashSet<Pos> {
    let on_ground_cell_positions = scan_connection_on_ground(field);
    let mut floating_cell_positions = HashSet::new();

    for row in field.rows() {
        for cell_ref in row.cell_refs() {
            let pos = cell_ref.pos();
            if !cell_ref.cell().is_empty() && !on_ground_cell_positions.contains(&pos) {
                floating_cell_positions.insert(pos);
            }
        }
    }

    floating_cell_positions
}

fn scan_connection_on_ground(field: &Field) -> HashSet<Pos> {
    let mut positions = HashSet::new();

    for on_ground_cell in field.rows().last().unwrap().cell_refs() {
        scan_connection(field, on_ground_cell.pos(), &mut positions);
    }

    positions
}

fn scan_connection(field: &Field, current_pos: Pos, connected_positions: &mut HashSet<Pos>) {
    // 現在の注目セルが空でないセルで，かつまだラベル付けされていない場合にだけラベル付け
    if let Some(_) = field
        .get(current_pos)
        .take_if(|c| !c.is_empty())
        .take_if(|_| !connected_positions.contains(&current_pos))
    {
        connected_positions.insert(current_pos);
        // 周囲のセルのラベル付け
        scan_connection(field, current_pos + right(1), connected_positions);
        scan_connection(field, current_pos + left(1), connected_positions);
        scan_connection(field, current_pos + below(1), connected_positions);
        scan_connection(field, current_pos + above(1), connected_positions);
    }
}
