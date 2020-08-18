mod data_type;
mod game;
mod geometry;
mod graphics;
mod user;

use game::field_animation::Drawer;
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

impl<'t> Drawer for StdoutDrawer<'t> {
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
