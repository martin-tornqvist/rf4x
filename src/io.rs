// #![allow(dead_code)]

extern crate ncurses;

use cmn_types::*;

pub type Clr = i32;

pub const CLR_BLK: i32 = ncurses::COLOR_BLACK as i32;
pub const CLR_RED: i32 = ncurses::COLOR_RED as i32;
pub const CLR_GRN: i32 = ncurses::COLOR_GREEN as i32;
pub const CLR_YEL: i32 = ncurses::COLOR_YELLOW as i32;
pub const CLR_BLU: i32 = ncurses::COLOR_BLUE as i32;
pub const CLR_MAG: i32 = ncurses::COLOR_MAGENTA as i32;
pub const CLR_CYA: i32 = ncurses::COLOR_CYAN as i32;
pub const CLR_WHI: i32 = ncurses::COLOR_WHITE as i32;

pub const FONT_NORM: bool = false;
pub const FONT_BOLD: bool = true;

pub fn init()
{
    // Setup ncurses
    ncurses::initscr();

    // TODO: What does this do?
    ncurses::raw();

    // Allow for extended keyboard (like F1)
    ncurses::keypad(ncurses::stdscr, true);

    // Don't echo characters to the window
    ncurses::noecho();

    // Hide the cursor
    ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // Use colors
    ncurses::start_color();

    // Init color pairs
    for f in 0..ncurses::COLORS {
        for b in 0..ncurses::COLORS {

            let idx: i16 = (f * ncurses::COLORS + b) as i16;

            ncurses::init_pair(idx, f as i16, b as i16);
        }
    }
}

pub fn cleanup()
{
    ncurses::endwin();
}

pub fn update_scr()
{
    ncurses::refresh();
}

pub fn clear_scr()
{
    ncurses::clear();
}

pub fn scr_dim() -> P
{
    let mut p = P { ..Default::default() };
    ncurses::getmaxyx(ncurses::stdscr, &mut p.y, &mut p.x);
    p
}

pub fn draw_char(p: P, c: char, fg: Clr, bg: Clr, is_bold: bool)
{
    set_clr_attr(fg, bg);

    if is_bold {
        ncurses::attron(ncurses::A_BOLD());
    }

    ncurses::mvaddch(p.y, p.x, c as u32);

    if is_bold {
        ncurses::attroff(ncurses::A_BOLD());
    }

    // ncurses::attroff(ncurses::COLOR_PAIR(1));
}

pub fn draw_text(p: P, text: &str, fg: Clr, bg: Clr, is_bold: bool)
{
    set_clr_attr(fg, bg);

    if is_bold {
        ncurses::attron(ncurses::A_BOLD());
    }

    ncurses::mvprintw(p.y, p.x, text);

    if is_bold {
        ncurses::attroff(ncurses::A_BOLD());
    }

    // ncurses::attroff(ncurses::COLOR_PAIR(1));
}

pub fn get_input() -> i32
{
    ncurses::getch()
}

fn set_clr_attr(fg: Clr, bg: Clr)
{
    let idx: i16 = (fg * ncurses::COLORS + bg) as i16;

    let ncurses_clr_pair = ncurses::COLOR_PAIR(idx);

    ncurses::attron(ncurses_clr_pair);
}
