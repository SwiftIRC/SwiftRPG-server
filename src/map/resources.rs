use crate::map::data::Noise;

pub fn trees(noises: &Vec<Noise>, point: [f64; 2], biome: &str) -> i32 {
    let trees: i32;
    let tree_modifier: i32 = match biome {
        "rainforest" => 5,
        "forest" => 3,
        "grassland" | "mountain" | "snow" => 2,
        _ => 0,
    };

    let humidity = noises[2].get(point) + 1.0;

    trees = (humidity * 4.0) as i32;

    println!("Trees: {} || Modifier: {}", trees, tree_modifier);

    trees * tree_modifier
}

pub fn rocks(noises: &Vec<Noise>, point: [f64; 2], biome: &str) -> i32 {
    let rocks: i32;
    let rock_modifier: i32 = match biome {
        "mountain" => 5,
        "snow" | "tundra" => 2,
        "grassland" | "forest" | "rainforest" => 1,
        _ => 0,
    };

    let height = noises[0].get(point) + 1.0;

    rocks = (height * 4.0) as i32;

    println!("Rocks: {} || Modifier: {}", rocks, rock_modifier);

    rocks * rock_modifier
}
