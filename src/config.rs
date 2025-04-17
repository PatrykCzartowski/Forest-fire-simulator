use crate::get_user_input;

pub struct Config {
    pub forest_size: i32,
    pub desired_forest_density: f32,
    pub current_forest_density: f32,
    
    pub desired_water_density: f32,
    pub current_water_density: f32,
    
    pub tree_count: i32,
    pub original_tree_count: i32,
    pub simulation_speed_ms: u64,

    pub pine_count: i32,
    pub oak_count: i32,
    pub birch_count: i32,
    pub redwood_count: i32,
}

impl Config {
    pub fn new() -> Self {
        println!("Forest Fire Simulator Setup\n");

        let forest_size: i32 = loop {
            let input = get_user_input("Enter forest size (10-100): ");
            match input.parse::<i32>() {
                Ok(size) if size >= 10 && size <= 100 => break size,
                _ => println!("Please enter a number between 10 and 100"),
            }
        };

        let desired_forest_density: f32 = loop {
            let input = get_user_input("Enter desired forest density (0.1-1.0): ");
            match input.parse::<f32>() {
                Ok(density) if density >= 0.1 && density <= 1.0 => break density,
                _ => println!("Please enter a number between 0.1 and 1.0"),
            }
        };

        let desired_water_density: f32 = loop {
            let input = get_user_input("Enter desired water density (0.0-0.5): ");
            match input.parse::<f32>() {
                Ok(density) if density >= 0.0 && density <= 0.5 => break density,
                _ => println!("Please enter a number between 0.0 and 0.5"),
            }
        };

        Self {
            forest_size,
            desired_forest_density,
            current_forest_density: 0.0,
            desired_water_density,
            current_water_density: 0.0,
            tree_count: 0,
            original_tree_count: 0,
            simulation_speed_ms: 100,
            pine_count: 0,
            oak_count: 0,
            birch_count: 0,
            redwood_count: 0,
        }
    }

}