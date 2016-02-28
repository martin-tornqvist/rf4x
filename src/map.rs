use cmn_types::*;

pub const MAP_SIZE: P = P { x: 16, y: 8 };

pub const TER_GRD : i32 = 0;
pub const TER_MNT : i32 = 1;

pub struct Map
{
    pub terrain: [[i32; MAP_SIZE.y as usize]; MAP_SIZE.x as usize],
}

impl Default for Map
{
    fn default() -> Map
    {
        Map { terrain: [[TER_GRD; MAP_SIZE.y as usize]; MAP_SIZE.x as usize] }
    }
}
