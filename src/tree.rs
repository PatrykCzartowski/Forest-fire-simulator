#[derive(Clone, Copy, PartialEq)]
pub enum TreeStatus {
    Alive,
    Struck,
    Kindling,
    Burning,
    Burned,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TreeType {
    Pine,    // Highly flammable, burns quickly
    Oak,     // Less flammable, burns slowly
    Birch,   // Medium flammability
    Redwood, // Fire resistant
}

#[derive(Clone)]
pub struct Tree {
    pub status: TreeStatus,
    pub tree_type: TreeType,
    pub flammability: f32,
    pub burn_time: u32,
}

impl Tree {
    pub fn new() -> Self {
        let tree_type = match rand::random::<u8>() % 4 {
            0 => TreeType::Pine,
            1 => TreeType::Oak,
            2 => TreeType::Birch,
            _ => TreeType::Redwood,
        };

        let (flammability, burn_time) = match tree_type {
            TreeType::Pine => (0.9, 1),      
            TreeType::Oak => (0.5, 3),       
            TreeType::Birch => (0.7, 2),     
            TreeType::Redwood => (0.3, 4),
        };

        Tree {
            status: TreeStatus::Alive,
            tree_type,
            flammability,
            burn_time,
        }
    }
}