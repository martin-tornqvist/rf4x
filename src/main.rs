mod io;
use io::*;

mod cmn_types;
use cmn_types::*;

fn main()
{
    println!("main()");

    io::init();

    loop {
        io::clear_scr();

        let scr_dim = io::scr_dim();

        for x in 0..scr_dim.x {
            for y in 0..scr_dim.y {
                io::draw_char(P { x: x, y: y }, '.', CLR_GRN, CLR_BLK, FONT_NORM);
            }
        }

        io::draw_text(P { x: 1, y: 1 },
                      &format!("Some number: {}", 42),
                      CLR_WHI,
                      CLR_BLK,
                      FONT_BOLD);

        io::update_scr();

        let inp = io::get_input();

        if inp == 'q' as i32 {
            break;
        }
    }

    io::cleanup();

    println!("main() [DONE]");
}
