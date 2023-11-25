use serde::{Deserialize, Serialize};

use crate::map::data::Noise;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Resources {
    pub trees: i32,
    pub rocks: i32,
    pub herbs: i32,
    pub fruits: i32,
}

impl Resources {
    pub fn new(noises: &Vec<Noise>, point: [f64; 2], biome: &str) -> Self {
        Self {
            trees: trees(noises, point, biome),
            rocks: rocks(noises, point, biome),
            herbs: herbs(noises, point, biome),
            fruits: fruits(noises, point, biome),
        }
    }
}

pub fn trees(noises: &Vec<Noise>, point: [f64; 2], biome: &str) -> i32 {
    let trees: i32;
    let tree_modifier: i32 = match biome {
        "rainforest" => 5,
        "forest" => 3,
        "grassland" | "mountain" => 2,
        "snow" => 1,
        _ => 0,
    };

    let humidity = noises[2].get(point) + 1.0;

    trees = (humidity * 4.0) as i32;

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

    rocks * rock_modifier
}

pub fn herbs(noises: &Vec<Noise>, point: [f64; 2], biome: &str) -> i32 {
    let herbs: i32;
    let herb_modifier: i32 = match biome {
        "rainforest" => 5,
        "forest" => 3,
        "grassland" | "mountain" => 2,
        "tundra" | "snow" => 1,
        _ => 0,
    };

    let humidity = noises[2].get(point) + 1.0;

    herbs = (humidity * 3.0) as i32;

    herbs * herb_modifier
}

pub fn fruits(noises: &Vec<Noise>, point: [f64; 2], biome: &str) -> i32 {
    let fruits: i32;
    let fruit_modifier: i32 = match biome {
        "rainforest" => 5,
        "forest" => 3,
        "grassland" | "mountain" => 2,
        _ => 0,
    };

    let humidity = noises[2].get(point) + 1.0;

    fruits = (humidity * 4.0) as i32;

    fruits * fruit_modifier
}
