use crate::map::data::Noise;

pub fn trees(noises: &Vec<Noise>, point: [f64; 2], biome: &str) -> i32 {
    let trees: i32;
    let tree_modifier: i32;

    if biome == "rainforest" {
        tree_modifier = 5;
    } else if biome == "forest" {
        tree_modifier = 3;
    } else if biome == "grassland" || biome == "mountain" || biome == "snow" {
        tree_modifier = 2;
    } else {
        tree_modifier = 0;
    }

    let humidity = noises[2].get(point) + 1.0;

    trees = (humidity * 4.0) as i32;

    println!("Trees: {} || Modifier: {}", trees, tree_modifier);

    trees * tree_modifier
}
