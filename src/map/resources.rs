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
