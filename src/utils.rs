use std::ops::Sub;
use std::ops::Add;
use std::cmp::PartialEq;

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

// -----------------------------------------------------------------------------
// Position
// -----------------------------------------------------------------------------
#[derive(Copy)]
pub struct P
{
    pub x: i32,
    pub y: i32,
}

impl P
{
    pub fn new() -> P
    {
        P { x: 0, y: 0 }
    }

    pub fn new_xy(x: i32, y: i32) -> P
    {
        P { x: x, y: y }
    }

    pub fn offs(&self, v: i32) -> P
    {
        P::new_xy(self.x + v, self.y + v)
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

impl PartialEq for P
{
    fn eq(&self, other: &P) -> bool
    {
        self.x == other.x && self.y == other.y
    }
}

impl Add for P
{
    type Output = P;

    fn add(self, other: P) -> P
    {
        P {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for P
{
    type Output = P;

    fn sub(self, other: P) -> P
    {
        P {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub fn p_offset(dir: Dir, p: &mut P)
{
    let d = to_offset(dir);

    *p = *p + d;
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

// -----------------------------------------------------------------------------
// Rectangle
// -----------------------------------------------------------------------------
#[derive(Copy)]
pub struct R
{
    pub p0: P,
    pub p1: P,
}

impl R
{
    pub fn new() -> R
    {
        R {
            p0: P::new(),
            p1: P::new(),
        }
    }

    pub fn new_pp(p0: &P, p1: &P) -> R
    {
        R { p0: *p0, p1: *p1 }
    }
}

impl Clone for R
{
    fn clone(&self) -> R
    {
        R {
            p0: self.p0,
            p1: self.p1,
        }
    }
}

impl PartialEq for R
{
    fn eq(&self, other: &R) -> bool
    {
        self.p0 == other.p0 && self.p1 == other.p1
    }
}

// -----------------------------------------------------------------------------
// Test cases
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p()
    {
        // Verify copy
        let p0 = P::new_xy(3, 5);

        let p1 = p0;

        assert_eq!(p0.x, p1.x);
        assert_eq!(p0.y, p1.y);

        // Verify comparison
        assert!(p0 == p1);

        // Verify offset by value
        let p2 = p0.offs(100);

        assert_eq!(103, p2.x);
        assert_eq!(105, p2.y);

        // Verify addition
        let p3 = p0 + P::new_xy(200, 300);

        assert_eq!(203, p3.x);
        assert_eq!(305, p3.y);

        // Verify subtraction
        let p4 = p0 - P::new_xy(10, 1);

        assert_eq!(-7, p4.x);
        assert_eq!(4, p4.y);
    }
}
