extern crate image;
extern crate noise;

use noise::*;
use rocket::serde::{Deserialize, Serialize};

pub struct NoiseTemplate {
    pub name: String,
    pub frequency: f64,
    pub persistence: f64,
    pub lacunarity: f64,
    pub octaves: usize,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Noise {
    pub noise: Fbm<Perlin>,
    pub name: String,
}

impl Noise {
    pub fn new(noise: Fbm<Perlin>, name: String) -> Self {
        Self { noise, name }
    }

    pub fn get(&self, point: [f64; 2]) -> f64 {
        self.noise.get(point)
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Point {
    point: (i32, i32),
    converted_point: Vec<f64>,
    height: f64,
    biome: String,
    trees: i32,
    rocks: i32,
}

impl Point {
    pub fn new(
        point: (i32, i32),
        converted_point: Vec<f64>,
        height: f64,
        biome: String,
        trees: i32,
        rocks: i32,
    ) -> Point {
        Point {
            point: point,
            converted_point: converted_point,
            height: height,
            biome: biome,
            trees: trees,
            rocks: rocks,
        }
    }
}
