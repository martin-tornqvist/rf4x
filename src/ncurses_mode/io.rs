extern crate ncurses;

use cmn_types::*;

// -----------------------------------------------------------------------------
// Colors and font
// -----------------------------------------------------------------------------
pub enum Clr
{
    Black = ncurses::COLOR_BLACK as isize,
    Red = ncurses::COLOR_RED as isize,
    Green = ncurses::COLOR_GREEN as isize,
    Yellow = ncurses::COLOR_YELLOW as isize,
    Blue = ncurses::COLOR_BLUE as isize,
    Magenta = ncurses::COLOR_MAGENTA as isize,
    Cyan = ncurses::COLOR_CYAN as isize,
    White = ncurses::COLOR_WHITE as isize,
}

#[derive(Clone, Copy)]
pub enum FontWgt
{
    Normal = 0,
    Bold,
}

// -----------------------------------------------------------------------------
// Keyboard key codes
// -----------------------------------------------------------------------------
pub const KEY_DOWN: i32 = ncurses::KEY_DOWN;
pub const KEY_UP: i32 = ncurses::KEY_UP;
pub const KEY_LEFT: i32 = ncurses::KEY_LEFT;
pub const KEY_RIGHT: i32 = ncurses::KEY_RIGHT;
pub const KEY_HOME: i32 = ncurses::KEY_HOME;
pub const KEY_BACKSPACE: i32 = ncurses::KEY_BACKSPACE;
pub const KEY_F0: i32 = ncurses::KEY_F0;
pub const KEY_F1: i32 = ncurses::KEY_F1;
pub const KEY_F2: i32 = ncurses::KEY_F2;
pub const KEY_F3: i32 = ncurses::KEY_F3;
pub const KEY_F4: i32 = ncurses::KEY_F4;
pub const KEY_F5: i32 = ncurses::KEY_F5;
pub const KEY_F6: i32 = ncurses::KEY_F6;
pub const KEY_F7: i32 = ncurses::KEY_F7;
pub const KEY_F8: i32 = ncurses::KEY_F8;
pub const KEY_F9: i32 = ncurses::KEY_F9;
pub const KEY_F10: i32 = ncurses::KEY_F10;
pub const KEY_F11: i32 = ncurses::KEY_F11;
pub const KEY_F12: i32 = ncurses::KEY_F12;
pub const KEY_F13: i32 = ncurses::KEY_F13;
pub const KEY_F14: i32 = ncurses::KEY_F14;
pub const KEY_F15: i32 = ncurses::KEY_F15;
pub const KEY_DC: i32 = ncurses::KEY_DC;
pub const KEY_IC: i32 = ncurses::KEY_IC;
pub const KEY_SF: i32 = ncurses::KEY_SF;
pub const KEY_SR: i32 = ncurses::KEY_SR;
pub const KEY_NPAGE: i32 = ncurses::KEY_NPAGE;
pub const KEY_PPAGE: i32 = ncurses::KEY_PPAGE;
pub const KEY_ENTER: i32 = ncurses::KEY_ENTER;
pub const KEY_PRINT: i32 = ncurses::KEY_PRINT;
pub const KEY_LL: i32 = ncurses::KEY_LL;
pub const KEY_A1: i32 = ncurses::KEY_A1;
pub const KEY_A3: i32 = ncurses::KEY_A3;
pub const KEY_B2: i32 = ncurses::KEY_B2;
pub const KEY_C1: i32 = ncurses::KEY_C1;
pub const KEY_C3: i32 = ncurses::KEY_C3;
pub const KEY_BTAB: i32 = ncurses::KEY_BTAB;
pub const KEY_END: i32 = ncurses::KEY_END;
pub const KEY_MOUSE: i32 = ncurses::KEY_MOUSE;
pub const KEY_RESIZE: i32 = ncurses::KEY_RESIZE;
pub const KEY_EVENT: i32 = ncurses::KEY_EVENT;

// -----------------------------------------------------------------------------
// Public functions
// -----------------------------------------------------------------------------
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

pub fn draw_char(p: &P, c: char, fg: Clr, bg: Clr, bold: FontWgt)
{
    set_clr_attr(fg, bg);

    if bold as i32 == FontWgt::Bold as i32 {
        ncurses::attron(ncurses::A_BOLD());
    }

    ncurses::mvaddch(p.y, p.x, c as u32);

    if bold as i32 == FontWgt::Bold as i32 {
        ncurses::attroff(ncurses::A_BOLD());
    }
}

pub fn draw_text(p: &P, text: &str, fg: Clr, bg: Clr, bold: FontWgt)
{
    set_clr_attr(fg, bg);

    if bold as i32 == FontWgt::Bold as i32 {
        ncurses::attron(ncurses::A_BOLD());
    }

    ncurses::mvprintw(p.y, p.x, text);

    if bold as i32 == FontWgt::Bold as i32 {
        ncurses::attroff(ncurses::A_BOLD());
    }
}

pub fn get_input() -> (char, i32)
{
    let inp: i32 = ncurses::getch();

    let mut char_out = '\n';
    let mut code_out = 0;

    if inp >= 33 && inp <= 126 {
        char_out = (inp as u8) as char;
    } else {
        code_out = inp;
    }

    (char_out, code_out)
}

// -----------------------------------------------------------------------------
// Private functions
// -----------------------------------------------------------------------------
fn set_clr_attr(fg: Clr, bg: Clr)
{
    let idx = (fg as i32 * ncurses::COLORS + bg as i32) as i16;

    let ncurses_clr_pair = ncurses::COLOR_PAIR(idx);

    ncurses::attron(ncurses_clr_pair);
}
