#[cfg(unix)]
mod ncurses_mode;
#[cfg(unix)]
use ncurses_mode::io;

mod cmn_types;
use cmn_types::*;

mod cursor;

fn main()
{
    println!("main()");

    io::init();

    let mut cursor_p = P { ..P::default() };

    io::clear_scr();

    // ------------------------------------------------------------------------
    // Game loop
    // ------------------------------------------------------------------------
    loop {
        let scr_dim = io::scr_dim();

        for x in 0..scr_dim.x {
            for y in 0..scr_dim.y {
                io::draw_char(&P { x: x, y: y },
                              '.',
                              io::CLR_GRN,
                              io::CLR_BLK,
                              io::FONT_NORM);
            }
        }

        io::draw_text(&P { x: 1, y: 1 },
                      &format!("Some number: {}", 42),
                      io::CLR_WHI,
                      io::CLR_BLK,
                      io::FONT_BOLD);

        io::draw_char(&cursor_p, 'X', io::CLR_GRN, io::CLR_BLK, true);

        io::update_scr();

        let inp = io::get_input();

        match inp {
            ('q', 0) => {
                break;
            }

            ('6', 0) | ('\n', io::KEY_RIGHT) => {
                cursor::offset(Dir::Right, &mut cursor_p);
            }

            ('4', 0) | ('\n', io::KEY_LEFT) => {
                cursor::offset(Dir::Left, &mut cursor_p);
            }

            ('2', 0) | ('\n', io::KEY_DOWN) => {
                cursor::offset(Dir::Down, &mut cursor_p);
            }

            ('8', 0) | ('\n', io::KEY_UP) => {
                cursor::offset(Dir::Up, &mut cursor_p);
            }

            ('3', 0) => {
                cursor::offset(Dir::DownRight, &mut cursor_p);
            }

            ('9', 0) => {
                cursor::offset(Dir::UpRight, &mut cursor_p);
            }

            ('1', 0) => {
                cursor::offset(Dir::DownLeft, &mut cursor_p);
            }

            ('7', 0) => {
                cursor::offset(Dir::UpLeft, &mut cursor_p);
            }

            _ => {}
        }
    }

    io::cleanup();

    println!("main() [DONE]");
}
