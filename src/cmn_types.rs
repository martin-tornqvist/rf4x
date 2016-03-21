pub enum Dir
{
    Right,
    Left,
    Down,
    Up,
    DownRight,
    UpRight,
    DownLeft,
    UpLeft,
}

pub struct P
{
    pub x: i32,
    pub y: i32,
}

impl P
{
    pub fn new(x: i32, y: i32) -> P
    {
        P { x: x, y: y }
    }
}

impl Default for P
{
    fn default() -> P
    {
        P { x: 0, y: 0 }
    }
}

impl Clone for P
{
    fn clone(&self) -> P
    {
        P {
            x: self.x,
            y: self.y,
        }
    }
}

pub fn p_sum(p1: &P, p2: &P) -> P
{
    P {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    }
}

pub fn p_offset(dir: Dir, p: &mut P)
{
    let d = to_offset(dir);

    *p = p_sum(p, &d);
}

pub fn to_offset(dir: Dir) -> P
{
    match dir {
        Dir::Right => P { x: 1, y: 0 },
        Dir::Left => P { x: -1, y: 0 },
        Dir::Down => P { x: 0, y: 1 },
        Dir::Up => P { x: 0, y: -1 },
        Dir::DownRight => P { x: 1, y: 1 },
        Dir::UpRight => P { x: 1, y: -1 },
        Dir::DownLeft => P { x: -1, y: 1 },
        Dir::UpLeft => P { x: -1, y: -1 },
    }
}
