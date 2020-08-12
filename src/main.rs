mod data_type;
mod game;
mod geometry;
mod graphics;
mod user;

use game::BlockSelector;
use geometry::*;
use graphics::*;

fn main() {
    let mut canvas = RootCanvas::new();
    let term = console::Term::buffered_stdout();

    let input_mapper = user::SinglePlayerInputMapper;

    let mut block_selector = BlockFactory;

    let mut agent_field = game::AgentField::new(&mut block_selector);

    let mut buffer = String::new();

    loop {
        use user::GameCommand::*;

        let game_command = match term.read_key().ok().and_then(|key| input_mapper.map(key)) {
            Some(command) => command,
            None => continue,
        };

        let next_agent_field = match game_command {
            Left => agent_field.move_block_to_left(),
            Right => agent_field.move_block_to_right(),
            Down => agent_field.move_block_down(),
            Drop => agent_field.drop_block(),
            RotateClockwise => agent_field.rotate_block_clockwise(),
            RotateUnticlockwise => agent_field.rotate_block_unticlockwise(),
            _ => agent_field,
        };

        agent_field = next_agent_field;

        canvas.clear();

        agent_field.draw_on_child(Pos::origin(), &mut canvas);

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
