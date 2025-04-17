use crate::tree::Tree;

#[derive(Clone, PartialEq)]
pub enum TileType {
    Grass,
    Water,
}

#[derive(Clone)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub tile_type: TileType,
    pub tile_entity: Option<Tree>,
}

impl Tile {
    pub fn new(x: i32, y:i32, tile_type: TileType) -> Self {
        Tile {
            x,
            y,
            tile_type,
            tile_entity: None,
        }
    }

    pub fn set_entity(&mut self, entity: Tree) {
        self.tile_entity = Some(entity);
    }

}