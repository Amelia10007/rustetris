use super::*;
use crate::game::Cell;
use crate::graphics::Canvas;

pub enum ConnectBombInitResult {
    Connects(ConnectBomb),
    Stay(AnimationField),
}

pub struct ConnectBomb {
    field: AnimationField,
    frame: ConnectionAnimationFrame,
    new_big_bomb_upper_left_positions: Vec<Pos>,
}

impl ConnectBomb {
    pub fn new(field: AnimationField) -> ConnectBombInitResult {
        let new_big_bomb_upper_left_positions = (0..field.field.height())
            .flat_map(|y| {
                (0..field.field.width())
                    .map(move |x| Pos(PosX::right(x as i8), PosY::below(y as i8)))
            })
            .map(|upper_left| big_bomb_positions(upper_left))
            .filter(|positions| {
                positions
                    .iter()
                    .all(|&p| matches!(field.field.get(p), Some(&Cell::Bomb)))
            })
            .map(|positions| positions[0])
            .collect::<Vec<_>>();

        if new_big_bomb_upper_left_positions.is_empty() {
            ConnectBombInitResult::Stay(field)
        } else {
            let frame = ConnectionAnimationFrame::new();

            let state = Self {
                field,
                frame,
                new_big_bomb_upper_left_positions,
            };
            ConnectBombInitResult::Connects(state)
        }
    }
}

impl Animation for ConnectBomb {
    type Finished = AnimationField;

    fn wait_next(mut self) -> AnimationResult<Self, Self::Finished> {
        match self.frame.wait_next() {
            Some(frame) => {
                // アニメーションの遷移が「ボム連結中」になった段階で，フィールドを初めて書き換える
                if let ConnectionAnimationFrame::Connecting(_) = frame {
                    for &upper_left in self.new_big_bomb_upper_left_positions.iter() {
                        let upper_right = upper_left + right(1);
                        let lower_left = upper_left + below(1);
                        let lower_right = upper_left + right(1) + below(1);
                        *self.field.field.get_mut(upper_left).unwrap() = Cell::BigBombUpperLeft;
                        *self.field.field.get_mut(upper_right).unwrap() = Cell::BigBombUpperRight;
                        *self.field.field.get_mut(lower_left).unwrap() = Cell::BigBombLowerLeft;
                        *self.field.field.get_mut(lower_right).unwrap() = Cell::BigBombLowerRight;
                    }
                }
                AnimationResult::InProgress(Self { frame, ..self })
            }
            None => AnimationResult::Finished(self.field),
        }
    }

    fn draw<C: Canvas>(&self, canvas: &mut C) {
        self.field.draw(canvas);

        if let ConnectionAnimationFrame::Connecting(frame) = &self.frame {
            let canvas_cell = {
                use Color::*;
                let color = CanvasCellColor::new(Red, Black);
                let c = if frame.current_frame() % 2 == 0 {
                    'x'
                } else {
                    '+'
                };
                CanvasCell::new(SquareChar::new(c, c), color)
            };

            for &big_bomb_upper_left in self.new_big_bomb_upper_left_positions.iter() {
                for &pos in big_bomb_positions(big_bomb_upper_left).iter() {
                    canvas.draw_cell(pos, canvas_cell);
                }
            }
        }
    }
}

enum ConnectionAnimationFrame {
    Unconnect(AnimationFrame),
    Connecting(AnimationFrame),
    Connected(AnimationFrame),
}

impl ConnectionAnimationFrame {
    fn new() -> ConnectionAnimationFrame {
        ConnectionAnimationFrame::Unconnect(AnimationFrame::with_frame_count(5))
    }

    fn wait_next(self) -> Option<ConnectionAnimationFrame> {
        use ConnectionAnimationFrame::*;

        match self {
            Unconnect(frame) => match frame.wait_next() {
                Some(frame) => Some(Unconnect(frame)),
                None => Some(Connecting(AnimationFrame::with_frame_count(5))),
            },
            Connecting(frame) => match frame.wait_next() {
                Some(frame) => Some(Connecting(frame)),
                None => Some(Connected(AnimationFrame::with_frame_count(5))),
            },
            Connected(frame) => match frame.wait_next() {
                Some(frame) => Some(Connected(frame)),
                None => None,
            },
        }
    }
}

fn big_bomb_positions(upper_left: Pos) -> [Pos; 4] {
    let upper_right = upper_left + right(1);
    let lower_left = upper_left + below(1);
    let lower_right = upper_left + right(1) + below(1);
    [upper_left, upper_right, lower_left, lower_right]
}
