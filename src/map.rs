use mon::*;

pub const MAP_W: usize = 256;
pub const MAP_H: usize = 256;

#[derive(Clone, Copy)]
pub enum Ter
{
    Floor,
    Wall,
}

pub struct Map
{
    pub ter: [[Ter; MAP_H]; MAP_W],
    pub monsters: Vec<Mon>,
}

impl Map
{
    pub fn new() -> Map
    {
        Map {
            ter: [[Ter::Wall; MAP_H]; MAP_W],
            monsters: vec![],
        }
    }
}
