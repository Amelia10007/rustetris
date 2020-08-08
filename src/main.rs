mod data_type;
mod game;
mod geometry;
mod graphics;
mod ncurses_wrapper;

use ncurses_wrapper::*;

fn main() {
    let mut nc = ncurses_wrapper::NcursesWrapper::new().unwrap();

    let color1 = ColorPair::new(Color::Yellow, Color::Black);
    let color2 = ColorPair::new(Color::Green, Color::Black);

    let mut count = 0;

    for i in 0.. {
        let color = if i % 2 == 0 { color1 } else { color2 };

        nc.erase().unwrap();

        let keys = nc.keys();
        if keys.is_empty() {
            count = 0;
        } else {
            count += 1;
        }

        nc.add_str(format!("count {}, {} keys", count, keys.len()), color)
            .unwrap();

        nc.refresh().unwrap();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
