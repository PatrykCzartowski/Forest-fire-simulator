mod tree;
mod tile;
mod config;
mod forest;
mod simulation;

use config::Config;
use tree::*;
use tile::*;
use forest::*;
use simulation::*;

fn clear_screen() {
    if cfg!(windows) {
        std::process::Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed")
            .wait()
            .expect("failed to wait");
    } else {
        print!("\x1B[2J\x1B[1;1H");
    }
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn calc_forest_density(forest: &Vec<Vec<Tile>>, tree_count: i32) -> f32 {
    let total_tiles = (forest.len() * forest[0].len()) as f32;
    tree_count as f32 / total_tiles
}

fn main() {
    clear_screen();
    let mut cfg = Config::new();
    let mut forest = init_forest(&mut cfg);
    run_simulation(&mut forest, &mut cfg);
}