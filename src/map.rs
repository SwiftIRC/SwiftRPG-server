extern crate image;
extern crate noise;

use ::core::ops::Sub;
use noise::{utils::*, *};
use num_traits::MulAdd;
use rocket::serde::{Deserialize, Serialize};

struct NoiseTemplate {
    name: String,
    frequency: f64,
    persistence: f64,
    lacunarity: f64,
    octaves: usize,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Noise {
    noise: Fbm<Perlin>,
    name: String,
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

pub fn generate_map(
    width: usize,
    height: usize,
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
    is_seamless: bool,
    seed: u32,
) -> (NoiseMap, Vec<Noise>) {
    let noises = generate_noises(seed);

    noises.iter().for_each(|n| {
        imagerender_mapbuilder_raw(&n.noise, &format!("{}.png", n.name));
    });

    let noise_map = build(width, height, x1, x2, y1, y2, is_seamless, seed);

    imagerender_mapbuilder(&noise_map, "merged-chunk.png");

    (noise_map, noises)
}

fn generate_noise_fn(seed: u32, t: &NoiseTemplate) -> Fbm<Perlin> {
    Fbm::<Perlin>::new(seed)
        .set_frequency(t.frequency)
        .set_persistence(t.persistence)
        .set_lacunarity(t.lacunarity)
        .set_octaves(t.octaves)
}

fn generate_noises(seed: u32) -> Vec<Noise> {
    let mut index = 0;

    get_map_types()
        .iter()
        .map(|t| {
            index += 1;

            Noise::new(generate_noise_fn(seed + index, t), t.name.to_owned())
        })
        .collect()
}

fn get_map_types() -> Vec<NoiseTemplate> {
    vec![
        NoiseTemplate {
            name: "height".to_string(),
            frequency: 0.7,
            persistence: 0.1,
            lacunarity: 2.1,
            octaves: 4,
        },
        NoiseTemplate {
            name: "temperature".to_string(),
            frequency: 0.3,
            persistence: 0.5,
            lacunarity: 1.8,
            octaves: 4,
        },
        NoiseTemplate {
            name: "humidity".to_string(),
            frequency: 0.2,
            persistence: 0.5,
            lacunarity: 2.0,
            octaves: 4,
        },
    ]
}

fn imagerender_mapbuilder_raw(noise_map: &noise::Fbm<noise::Perlin>, filename: &str) {
    ImageRenderer::new()
        .render(&mapbuilder_raw(&noise_map))
        .write_to_file(filename);
}

fn mapbuilder_raw(noise_map: &noise::Fbm<noise::Perlin>) -> NoiseMap {
    PlaneMapBuilder::<&noise::Cache<&noise::Fbm<noise::Perlin>>, 2>::new(&Cache::new(noise_map))
        .set_size(160, 160)
        .set_is_seamless(true)
        .set_x_bounds(0.0, 10.0)
        .set_y_bounds(0.0, 10.0)
        .build()
}

fn imagerender_mapbuilder(noise_map: &NoiseMap, filename: &str) {
    ImageRenderer::new()
        // .set_gradient(ColorGradient::new().build_terrain_gradient())
        .set_gradient(build_terrain_gradient())
        .render(noise_map)
        .write_to_file(filename);
}

fn build_terrain_gradient() -> ColorGradient {
    ColorGradient::new()
        .clear_gradient()
        .add_gradient_point(-0.9, [0, 0, 0, 255]) // deep ocean
        .add_gradient_point(-0.7, [0, 0, 255, 255]) // ocean
        .add_gradient_point(-0.5, [194, 178, 128, 255]) // sand
        .add_gradient_point(-0.25, [100, 100, 100, 255]) // tundra
        .add_gradient_point(0.0, [214, 198, 148, 255]) // desert
        .add_gradient_point(0.2, [5, 154, 23, 255]) // grassland
        .add_gradient_point(0.4, [0, 100, 0, 255]) // forest
        .add_gradient_point(0.6, [55, 128, 55, 255]) // rainforest
        .add_gradient_point(0.8, [180, 180, 180, 255]) // mountain
        .add_gradient_point(0.9, [255, 255, 255, 255]) // snow
}

#[allow(dead_code)]
pub fn reconstruct_map(
    width: usize,
    height: usize,
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
    is_seamless: bool,
    noises: &Vec<Noise>,
) -> NoiseMap {
    let mut noise = NoiseMap::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let point: f64 = get_point(
                [x as f64, y as f64],
                noises,
                width,
                height,
                x1,
                x2,
                y1,
                y2,
                is_seamless,
            );

            noise.set_value(x, y, point);
        }
    }
    ImageRenderer::new()
        // .set_gradient(ColorGradient::new().build_terrain_gradient())
        .set_gradient(build_terrain_gradient())
        .render(&noise)
        .write_to_file("debug-reconstructed.png");

    noise
}

pub fn convert_point(
    point: [f64; 2],
    noises: &Vec<Noise>,
    width: usize,
    height: usize,
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
    is_seamless: bool,
) -> Vec<f64> {
    let x_extent = x2 - x1;
    let y_extent = y2 - y1;

    let x_step = x_extent / width as f64;
    let y_step = y_extent / height as f64;

    let current_x = x1 + x_step * point[0];
    let current_y = y1 + y_step * point[1];

    let points = get_points([current_x, current_y], &noises);

    let final_value = if is_seamless {
        let sw_value = noises[0].get([current_x, current_y]);
        let se_value = noises[0].get([current_x + x_extent, current_y]);
        let nw_value = noises[0].get([current_x, current_y + y_extent]);
        let ne_value = noises[0].get([current_x + x_extent, current_y + y_extent]);

        let x_blend = 1.0 - ((current_x - x1) / x_extent);
        let y_blend = 1.0 - ((current_y - y1) / y_extent);

        let y0 = linear(sw_value, se_value, x_blend);
        let y1 = linear(nw_value, ne_value, x_blend);

        linear(y0, y1, y_blend)
    } else {
        noises[0].get([current_x, current_y])
    };

    vec![final_value, points[1], points[2]]
}

pub fn get_point(
    point: [f64; 2],
    noises: &Vec<Noise>,
    width: usize,
    height: usize,
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
    is_seamless: bool,
) -> f64 {
    let converted_point = convert_point(point, noises, width, height, x1, x2, y1, y2, is_seamless);

    get_biome_as_f64(get_biome(converted_point))
}

// Gets the point at the given coordinates for all noises,
// not run through the biome conversion
pub fn get_points(point: [f64; 2], noises: &Vec<Noise>) -> Vec<f64> {
    (0..noises.len()).map(|i| noises[i].get(point)).collect()
}

pub fn get_biome_as_f64(biome: String) -> f64 {
    match biome.as_str() {
        "deep_ocean" => -1.0,
        "ocean" => -0.7,
        "sand" => -0.5,
        "tundra" => 0.25,
        "desert" => 0.0,
        "grassland" => 0.2,
        "forest" => 0.4,
        "rainforest" => 0.6,
        "mountain" => 0.8,
        "snow" => 1.0,
        _ => 0.0,
    }
}

pub fn get_biome(points: Vec<f64>) -> String {
    let height = points[0];
    let temperature = points[1];
    let humidity = points[2];

    let biome: String;

    if height <= -0.3 {
        biome = "deep_ocean".to_string();
    } else if height <= -0.1 {
        biome = "ocean".to_string();
    } else if height <= 0.0 {
        biome = "sand".to_string();
    } else if height <= 0.5 {
        if temperature <= 0.0 {
            biome = "tundra".to_string();
        } else if temperature >= 0.7 {
            if humidity <= 0.0 {
                biome = "desert".to_string();
            } else if humidity <= 0.2 {
                biome = "grassland".to_string();
            } else if humidity <= 0.4 {
                biome = "forest".to_string();
            } else {
                biome = "rainforest".to_string();
            }
        } else if humidity <= 0.33 {
            biome = "grassland".to_string();
        } else if humidity >= 0.66 {
            biome = "forest".to_string();
        } else if height <= 0.2 {
            biome = "grassland".to_string();
        } else {
            biome = "forest".to_string();
        }
    } else if height <= 0.8 {
        biome = "mountain".to_string();
    } else {
        biome = "snow".to_string();
    }

    biome
}

fn build(
    width: usize,
    height: usize,
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
    is_seamless: bool,
    seed: u32,
) -> NoiseMap {
    let mut result_map = NoiseMap::new(width, height);

    let noises = generate_noises(seed);
    let noise = noises.first().unwrap();

    let x_bounds = (x1, x2);
    let y_bounds = (y1, y2);

    let x_extent = x_bounds.1 - x_bounds.0;
    let y_extent = y_bounds.1 - y_bounds.0;

    let x_step = x_extent / width as f64;
    let y_step = y_extent / height as f64;

    for y in 0..height {
        let current_y = y_bounds.0 + y_step * y as f64;

        for x in 0..width {
            let current_x = x_bounds.0 + x_step * x as f64;

            let points = get_points([current_x, current_y], &noises);

            let final_value = if is_seamless {
                let sw_value = noise.get([current_x, current_y]);
                let se_value = noise.get([current_x + x_extent, current_y]);
                let nw_value = noise.get([current_x, current_y + y_extent]);
                let ne_value = noise.get([current_x + x_extent, current_y + y_extent]);

                let x_blend = 1.0 - ((current_x - x_bounds.0) / x_extent);
                let y_blend = 1.0 - ((current_y - y_bounds.0) / y_extent);

                let y0 = linear(sw_value, se_value, x_blend);
                let y1 = linear(nw_value, ne_value, x_blend);

                linear(y0, y1, y_blend)
            } else {
                points[0]
            };

            result_map[(x, y)] =
                get_biome_as_f64(get_biome(vec![final_value, points[1], points[2]]));
        }
    }

    result_map
}

fn linear<T>(a: T, b: T, x: T) -> T
where
    T: MulAdd<Output = T> + Sub<Output = T> + Copy,
{
    x.mul_add(b - a, a)
}
