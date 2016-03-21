// TODO: Map should be bigger than this, and a viewport should be used
pub const MAP_W: usize = 80;
pub const MAP_H: usize = 20;

#[derive(Clone, Copy)]
pub enum Ter
{
    Floor,
    Wall,
}

pub struct Map
{
    pub ter: [[Ter; MAP_H]; MAP_W],
}

impl Default for Map
{
    fn default() -> Map
    {
        Map { ter: [[Ter::Wall; MAP_H]; MAP_W] }
    }
}
