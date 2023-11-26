extern crate image;
extern crate noise;

use crate::map::resources::*;
use noise::*;

use rocket::serde::{Deserialize, Serialize};

pub struct NoiseTemplate {
    pub name: String,
    pub frequency: f64,
    pub persistence: f64,
    pub lacunarity: f64,
    pub octaves: usize,
}

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

pub trait Copy {
    fn copy(&self) -> Self;
}

impl Copy for Noise {
    fn copy(&self) -> Self {
        Self {
            noise: self.noise.clone(),
            name: self.name.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Point {
    pub point: (i32, i32),
    pub converted_point: Vec<f64>,
    pub height: f64,
    pub biome: String,
    pub resources: Resources,
}

impl Point {
    pub fn new(
        point: (i32, i32),
        converted_point: Vec<f64>,
        height: f64,
        biome: String,
        resources: Resources,
    ) -> Point {
        Point {
            point: point,
            converted_point: converted_point,
            height: height,
            biome: biome,
            resources: resources,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Chunk {
    pub width: usize,
    pub height: usize,
    pub x1: f64,
    pub x2: f64,
    pub y1: f64,
    pub y2: f64,
    pub is_seamless: bool,
}

impl Chunk {
    pub fn new(
        width: usize,
        height: usize,
        x1: f64,
        x2: f64,
        y1: f64,
        y2: f64,
        is_seamless: bool,
    ) -> Chunk {
        Chunk {
            width: width,
            height: height,
            x1: x1,
            x2: x2,
            y1: y1,
            y2: y2,
            is_seamless: is_seamless,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Noises {
    pub height: Noise,
    pub temperature: Noise,
    pub humidity: Noise,
}

impl Noises {
    pub fn new(height: Noise, temperature: Noise, humidity: Noise) -> Self {
        Self {
            height,
            temperature,
            humidity,
        }
    }

    pub fn to_vec(&self) -> Vec<Noise> {
        vec![
            self.height.clone(),
            self.temperature.clone(),
            self.humidity.clone(),
        ]
    }

    pub fn len(&self) -> usize {
        3
    }

    pub fn get(&self, index: usize) -> &Noise {
        match index {
            0 => &self.height,
            1 => &self.temperature,
            2 => &self.humidity,
            _ => &self.height,
        }
    }
}
