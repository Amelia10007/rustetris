mod data_type;
mod game;
mod geometry;
mod graphics;
mod user;

use game::BlockSelector;
use geometry::*;
use graphics::*;
use user::Key::*;

fn main() {
    let mut canvas = RootCanvas::new();
    let term = console::Term::buffered_stdout();

    let mut buffer = String::new();

    let mut block_selector = BlockFactory;
    let mut block = block_selector.generate_block();
    let field = game::Field::empty();

    loop {
        match term.read_key() {
            Ok(key) => match key {
                Char('x') => block = block.rotate_clockwise(),
                Char('z') => block = block.rotate_unticlockwise(),
                Char('q') => break,
                _ => {}
            },
            _ => {}
        }

        canvas.clear();

        field.draw_on_child(Pos::origin(), &mut canvas);

        term.clear_screen().unwrap();
        canvas.construct_output_string(&mut buffer);
        term.write_str(&buffer).unwrap();
        term.flush().unwrap();
    }
}

struct BlockFactory;

impl BlockSelector for BlockFactory {
    fn select_block_shape(&mut self) -> game::BlockShape {
        game::QuadrupleBlockShape::J.into()
    }

    fn select_bomb(&mut self, _shape: game::BlockShape) -> game::BombTag {
        game::BombTag::None
    }
}
