// TODO: There is probably a better way to do this(?):
#[cfg(unix)]
mod ncurses_mode;
#[cfg(unix)]
use ncurses_mode::io;

mod cmn_types;
mod mon;
mod map;

use cmn_types::*;
use mon::*;
use map::*;

fn main()
{
    println!("main()");

    io::init();

    let mut player = Mon::new(&P::new(2, 4));

    let mut game_map = Map { ..Map::default() };

    for x in 1..MAP_W - 1 {
        for y in 1..MAP_H - 1 {
            game_map.ter[x][y] = Ter::Floor;
        }
    }

    io::clear_scr();

    // ------------------------------------------------------------------------
    // Game loop
    // ------------------------------------------------------------------------
    loop {
        // let scr_dim = io::scr_dim();

        for x in 0..MAP_W {
            for y in 0..MAP_H {

                let ch;
                let fg;
                let bg;

                let ter = game_map.ter[x][y];

                match ter {
                    Ter::Floor => {
                        ch = '.';
                        fg = io::Clr::White;
                        bg = io::Clr::Black;
                    }

                    Ter::Wall => {
                        ch = '#';
                        fg = io::Clr::White;
                        bg = io::Clr::Black;
                    }
                }

                io::draw_char(&P {
                                  x: x as i32,
                                  y: y as i32,
                              },
                              ch,
                              fg,
                              bg,
                              io::FontWgt::Normal);
            }
        }

        // io::draw_text(&P { x: 1, y: 1 },
        // &format!("Some number: {}", 42),
        // io::Clr::White,
        // io::Clr::Black,
        // io::FONT_BOLD);
        //

        io::draw_char(&player.p(),
                      '@',
                      io::Clr::White,
                      io::Clr::Black,
                      io::FontWgt::Bold);

        io::update_scr();

        let inp = io::get_input();

        match inp {
            ('q', 0) => {
                break;
            }

            ('6', 0) | ('\n', io::KEY_RIGHT) => {
                player.mv(Dir::Right);
            }

            ('4', 0) | ('\n', io::KEY_LEFT) => {
                player.mv(Dir::Left);
            }

            ('2', 0) | ('\n', io::KEY_DOWN) => {
                player.mv(Dir::Down);
            }

            ('8', 0) | ('\n', io::KEY_UP) => {
                player.mv(Dir::Up);
            }

            ('3', 0) => {
                player.mv(Dir::DownRight);
            }

            ('9', 0) => {
                player.mv(Dir::UpRight);
            }

            ('1', 0) => {
                player.mv(Dir::DownLeft);
            }

            ('7', 0) => {
                player.mv(Dir::UpLeft);
            }

            _ => {}
        }
    }

    io::cleanup();

    println!("main() [DONE]");
}
