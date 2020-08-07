mod data_type;
mod game;
mod geometry;
mod graphics;

use ncurses::*;

fn main() {
    /* If your locale env is unicode, you should use `setlocale`. */
    // let locale_conf = LcCategory::all;
    // setlocale(locale_conf, "zh_CN.UTF-8"); // if your locale is like mine(zh_CN.UTF-8).

    /* Start ncurses. */
    initscr();

    start_color();
    let color_index1 = 1;
    let foreground1 = COLOR_GREEN;
    let background1 = COLOR_YELLOW;
    let color_index2 = 2;
    let foreground2 = COLOR_BLUE;
    let background2 = COLOR_BLACK;
    init_pair(color_index1, foreground1, background1);
    init_pair(color_index2, foreground2, background2);

    /* Print to the back buffer. */
    addstr("Hello, world!");

    /* Print some unicode(Chinese) string. */
    // printw("Great Firewall dislike VPN protocol.\nGFW 不喜欢 VPN 协议。");

    /* Update the screen. */
    refresh();

    // non-blocking
    timeout(0);

    let mut current_color = color_index1;
    let mut count = 0;

    loop {
        const NO_INPUT: i32 = -1;
        const SPACE: i32 = 32;

        // check a key press
        match getch() {
            NO_INPUT => count += 1,
            SPACE => break,
            _ => {
                count = 0;

                current_color = if current_color == color_index1 {
                    color_index2
                } else {
                    color_index1
                };
            }
        }

        clear();
        attron(COLOR_PAIR(current_color));
        addstr(&format!("count: {}", count));

        attron(COLOR_PAIR(color_index1));
        addstr("hello");
        attron(COLOR_PAIR(color_index2));
        addstr("ncurses");

        refresh();

        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    /* Terminate ncurses. */
    endwin();
}
