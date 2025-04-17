use crate::{Config, Tile, Tree, TreeStatus, TreeType, calc_forest_density, clear_screen, forest::display_forest, forest::clear_forest};
use rand::prelude::*;
use std::collections::VecDeque;
use std::thread;
use std::time::Duration;

pub fn burn_adjacent_trees(forest: &mut Vec<Vec<Tile>>, x: i32, y: i32, cfg: &mut Config) -> i32 {
    if cfg.tree_count == 0 { return 0; }

    let forest_size = forest.len() as i32;
    let directions: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let mut burning_queue: VecDeque<(i32, i32, u32)> = VecDeque::new();
    let mut burned_count = 0;

    if let Some(tree) = &forest[x as usize][y as usize].tile_entity {
        if tree.status == TreeStatus::Alive {
            let tree_type = tree.tree_type;
            let flammability = tree.flammability;
            let burn_time = tree.burn_time;

            match tree_type {
                TreeType::Pine     =>  cfg.pine_count -= 1,
                TreeType::Oak      =>  cfg.oak_count -= 1,
                TreeType::Birch    =>  cfg.birch_count -= 1,
                TreeType::Redwood  =>  cfg.redwood_count -= 1,
            }

            forest[x as usize][y as usize].set_entity(Tree {
                status: TreeStatus::Struck,
                tree_type,
                flammability,
                burn_time,
            });

            clear_screen();
            display_forest(forest, cfg);
            println!("Lightning struck! Tree has been hit.");
            thread::sleep(Duration::from_millis(100));

            burning_queue.push_back((x, y, burn_time));
            burned_count += 1;
            cfg.tree_count -= 1;
        }
    }

    while !burning_queue.is_empty() {
        let (current_x, current_y, current_burn_time) = burning_queue.pop_front().unwrap();

        let is_original_struck_tree = current_x == x && current_y == y;

        if current_burn_time > 1 && !is_original_struck_tree {
            if let Some(tree) = &forest[current_x as usize][current_y as usize].tile_entity {
                let tree_type = tree.tree_type;
                let flammability = tree.flammability;

                forest[current_x as usize][current_y as usize].set_entity(Tree {
                    status: TreeStatus::Burning,
                    tree_type,
                    flammability,
                    burn_time: current_burn_time - 1,
                });

                burning_queue.push_back((current_x, current_y, current_burn_time - 1));
                continue;
            }
        }
        
        for (dx, dy) in directions.iter() {
            let new_x = current_x + dx;
            let new_y = current_y + dy;

            if new_x >= 0 && new_x < forest_size && new_y >= 0 && new_y < forest_size {
                if forest[new_x as usize][new_y as usize].tile_type == crate::tile::TileType::Water {
                    continue;
                }

                if let Some(tree) = &forest[new_x as usize][new_y as usize].tile_entity {
                    if tree.status == TreeStatus::Alive {
                        let probability = tree.flammability;

                        if rand::random::<f32>() <= probability {
                            let tree_type = tree.tree_type;
                            let flammability = tree.flammability;
                            let burn_time = tree.burn_time;

                            match tree_type {
                                TreeType::Pine     =>  cfg.pine_count -= 1,
                                TreeType::Oak      =>  cfg.oak_count -= 1,
                                TreeType::Birch    =>  cfg.birch_count -= 1,
                                TreeType::Redwood  =>  cfg.redwood_count -= 1,
                            }

                            forest[new_x as usize][new_y as usize].set_entity(Tree {
                                status: TreeStatus::Kindling,
                                tree_type,
                                flammability,
                                burn_time,
                            });

                            cfg.tree_count -= 1;
                            burning_queue.push_back((new_x, new_y, burn_time));
                            burned_count += 1;
                        }
                    } else if tree.status == TreeStatus::Kindling {
                        let tree_type = tree.tree_type;
                        let flammability = tree.flammability;
                        let burn_time = tree.burn_time;
                        
                        forest[new_x as usize][new_y as usize].set_entity(Tree {
                            status: TreeStatus::Burning,
                            tree_type,
                            flammability,
                            burn_time,
                        });
                    }
                }
            }
        }

        cfg.tree_count = cfg.original_tree_count - burned_count;
        cfg.current_forest_density = calc_forest_density(forest, cfg.tree_count);

        if !is_original_struck_tree {
            if let Some(tree) = &forest[current_x as usize][current_y as usize].tile_entity {
                let tree_type = tree.tree_type;
                let flammability = tree.flammability;
                
                forest[current_x as usize][current_y as usize].set_entity(Tree {
                    status: TreeStatus::Burned,
                    tree_type,
                    flammability,
                    burn_time: 0,
                });
            }
        }

        clear_screen();
        display_forest(forest, cfg);
        thread::sleep(Duration::from_millis(cfg.simulation_speed_ms));
    }
    
    burned_count
}

pub fn run_simulation(forest: &mut Vec<Vec<Tile>>, cfg: &mut Config) {
    let mut rng = rand::thread_rng();

    clear_screen();
    display_forest(&forest, &cfg);

    'lightning_loop: loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        input = input.trim().to_string();

        match input.as_str() {
            "q" | "Q" => break 'lightning_loop,
            _ => {
                let struck_x: i32 = rng.gen_range(0..cfg.forest_size);
                let struck_y: i32 = rng.gen_range(0..cfg.forest_size);

                clear_screen();
                println!("Lightning struck at ({}, {})", struck_x, struck_y);
                
                if let Some(tree) = &forest[struck_x as usize][struck_y as usize].tile_entity {
                    if tree.status == TreeStatus::Alive {
                        burn_adjacent_trees(forest, struck_x, struck_y, cfg);
                    } else {
                        clear_screen();
                        display_forest(&forest, &cfg);
                        println!("Lightning struck a non-living tree!");
                    }
                } else {
                    clear_screen();
                    display_forest(&forest, &cfg);
                    println!("No tree at the struck location!");
                }
            }
        }
        
        clear_screen();
        clear_forest(forest);
        display_forest(&forest, &cfg);
    }
}