mod data_type;
mod game;
mod geometry;
mod graphics;
mod user;

use game::*;
use graphics::*;

fn main() {
    let terminal = console::Term::buffered_stdout();

    let mut drawer = StdoutDrawer {
        terminal: &terminal,
        root_canvas: RootCanvas::new(),
    };

    let input_mapper = user::SinglePlayerInputMapper;

    let input = || loop {
        if let Ok(key) = terminal.read_key() {
            if let Some(command) = input_mapper.map(key) {
                break command;
            }
        }
    };

    game::single_play::execute_game(input, &mut drawer);
}

struct StdoutDrawer<'t> {
    terminal: &'t console::Term,
    root_canvas: RootCanvas,
}

impl<'t> crate::game::field_animation::Drawer for StdoutDrawer<'t> {
    type Canvas = RootCanvas;

    fn canvas_mut(&mut self) -> &mut Self::Canvas {
        &mut self.root_canvas
    }

    fn clear(&mut self) {
        self.root_canvas.clear();
        self.terminal.clear_screen().unwrap();
    }

    fn show(&mut self) {
        let mut buffer = String::new();
        self.root_canvas.construct_output_string(&mut buffer);
        self.terminal.write_str(&buffer).unwrap();
        self.terminal.flush().unwrap();
    }
}

struct QuadrupleBlockGenerator {
    current_index: usize,
}

impl BlockSelector for QuadrupleBlockGenerator {
    fn select_block_shape(&mut self) -> game::BlockShape {
        use game::QuadrupleBlockShape::*;

        let shapes = [O, J, L, Z, S, T, I];

        let shape = shapes[self.current_index % shapes.len()];
        self.current_index = (self.current_index + 1) % shapes.len();
        shape.into()
    }

    fn select_bomb(&mut self, _: game::BlockShape) -> game::BombTag {
        game::BombTag::Single(0)
    }
}
