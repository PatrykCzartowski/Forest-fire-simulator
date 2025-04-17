use crate::{Config, Tile, TileType, Tree, TreeType};
use rand::prelude::*;
use std::collections::VecDeque;
use colored::*;

pub fn create_water_body(forest: &mut Vec<Vec<Tile>>, start_x: i32, start_y: i32, size: i32, rng: &mut ThreadRng) {
    let mut water_queue = VecDeque::new();
    water_queue.push_back((start_x, start_y));
    let mut placed_water = 0;

    while let Some((x, y)) = water_queue.pop_front() {
        if placed_water >= size { break; }

        if x < 0 || y < 0 || x >= forest.len() as i32 || y >= forest[0].len() as i32 {
            continue;
        }

        forest[x as usize][y as usize] = Tile::new(x, y, TileType::Water);
        placed_water += 1;

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions.iter() {
            if rng.gen::<f32>() < 0.7 {
                water_queue.push_back((x + dx, y + dy));
            }
        }
    }
}

pub fn init_forest(cfg: &mut Config) -> Vec<Vec<Tile>> {
    let mut rng = rand::thread_rng();
    let mut forest: Vec<Vec<Tile>> = (0..cfg.forest_size)
        .map(|x| (0..cfg.forest_size)
            .map(|y| Tile::new(x, y, TileType::Grass))
            .collect())
        .collect();

    // Water
    while cfg.current_water_density < cfg.desired_water_density {
        let x = rng.gen_range(0..cfg.forest_size);
        let y = rng.gen_range(0..cfg.forest_size);
        
        let water_size = rng.gen_range(3..16);
        create_water_body(&mut forest, x, y, water_size, &mut rng);
        
        cfg.current_water_density = forest.iter().flatten()
            .filter(|tile| tile.tile_type == TileType::Water)
            .count() as f32 / (cfg.forest_size * cfg.forest_size) as f32;
    }

    // Trees
    while cfg.current_forest_density < cfg.desired_forest_density {
        let x: i32 = rng.gen_range(0..cfg.forest_size);
        let y: i32 = rng.gen_range(0..cfg.forest_size);
        
        if forest[x as usize][y as usize].tile_type == TileType::Grass {
            let new_tree = Tree::new();

            match new_tree.tree_type {
                TreeType::Pine      =>   cfg.pine_count += 1,
                TreeType::Oak       =>   cfg.oak_count += 1,
                TreeType::Birch     =>   cfg.birch_count += 1,
                TreeType::Redwood   =>   cfg.redwood_count += 1,
            }

            forest[x as usize][y as usize].set_entity(new_tree);
            cfg.tree_count += 1;
            cfg.current_forest_density += 1.0 / (cfg.forest_size * cfg.forest_size) as f32;
        }
    }

    cfg.original_tree_count = cfg.tree_count;
    forest
}

pub fn display_forest(forest: &Vec<Vec<Tile>>, cfg: &Config) {
    let burned_percent = ((cfg.original_tree_count - cfg.tree_count) as f32 / cfg.original_tree_count as f32 * 100.0).round();

    println!("Forest size: {}x{} | Original density: {:.2}% | Original number of trees: {}", 
             cfg.forest_size, cfg.forest_size, (cfg.desired_forest_density * 100.0).round(), cfg.original_tree_count);
    println!("Current density: {:.2}% | Remaining trees: {} | Burned: {:.2}% ", 
             (cfg.current_forest_density * 100.0), cfg.tree_count, burned_percent);
    println!("Water density: {:.2}% ", (cfg.current_water_density * 100.0));
    println!("Pine: {} | Oak: {} | Birch: {} | Redwood: {}", 
             cfg.pine_count, cfg.oak_count, cfg.birch_count, cfg.redwood_count);
    println!("Tree types: P = Pine, O = Oak, B = Birch, R = Redwood");
    println!("Legend: X = Struck, K = Kindling, F = Burning, # = Burned");
    println!("Press Enter for lightning, 'q' to quit");
    
    for row in forest {
        for tile in row {
            if tile.tile_entity.is_some() {
                let tree = tile.tile_entity.as_ref().unwrap();
                
                match tree.status {
                    crate::tree::TreeStatus::Alive => {
                        match tree.tree_type {
                            TreeType::Pine     =>  print!("{} ", "P".green()),
                            TreeType::Oak      =>  print!("{} ", "O".green()),
                            TreeType::Birch    =>  print!("{} ", "B".green()),
                            TreeType::Redwood  =>  print!("{} ", "R".green()),
                        }
                    },
                    crate::tree::TreeStatus::Struck    =>  print!("{} ", "X".white()),
                    crate::tree::TreeStatus::Kindling  =>  print!("{} ", "K".yellow()),
                    crate::tree::TreeStatus::Burning   =>  print!("{} ", "F".red()),
                    crate::tree::TreeStatus::Burned    =>  print!("{} ", "#".truecolor(128, 128, 128)),
                }
            } else {
                match tile.tile_type {
                    TileType::Water => print!("{} ", "~".blue()),
                    _ => print!("{} ", ".".truecolor(128, 128, 128)),
                }
            }
        }
        println!();
    }
}

pub fn clear_forest(forest: &mut Vec<Vec<Tile>>) {
    for row in forest.iter_mut() {
        for tile in row.iter_mut() {
            if let Some(tree) = &mut tile.tile_entity {
                // Don't change struck trees
                if tree.status != crate::tree::TreeStatus::Struck {
                    match tree.status {
                        crate::tree::TreeStatus::Kindling | crate::tree::TreeStatus::Burning => {
                            let tree_type = tree.tree_type;
                            let flammability = tree.flammability;
                            
                            tile.set_entity(Tree {
                                status: crate::tree::TreeStatus::Burned,
                                tree_type,
                                flammability,
                                burn_time: 0,
                            });
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}