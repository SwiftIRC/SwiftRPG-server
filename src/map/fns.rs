extern crate image;
extern crate noise;

use super::data::{Chunk, Coordinate, Noise, NoiseTemplate, Noises, Points};
use ::core::ops::Sub;
use noise::{utils::*, *};
use num_traits::MulAdd;
use std::collections::HashMap;

pub fn generate_map(chunk: &Chunk, noises: &Noises) -> NoiseMap {
    for noise in noises.to_vec() {
        imagerender_mapbuilder_raw(&noise.noise, &format!("{}.png", noise.name));
    }

    let noise_map = build(&chunk, noises);

    imagerender_mapbuilder(&noise_map, "merged-chunk.png");

    noise_map
}

fn generate_noise_fn(seed: u32, t: &NoiseTemplate) -> Fbm<Perlin> {
    Fbm::<Perlin>::new(seed)
        .set_frequency(t.frequency)
        .set_persistence(t.persistence)
        .set_lacunarity(t.lacunarity)
        .set_octaves(t.octaves)
}

pub fn generate_noises(seed: u32) -> Noises {
    let mut index = 0;

    let mut hashmap = HashMap::new();

    get_map_types().iter().for_each(|t| {
        index += 1;

        let noise = Noise::new(generate_noise_fn(seed + index, t), t.name.to_owned());

        hashmap.insert(t.name.to_owned(), noise);
    });

    Noises::new(
        hashmap["height"].clone(),
        hashmap["temperature"].clone(),
        hashmap["humidity"].clone(),
    )
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
    noises: &Noises,
) -> NoiseMap {
    let mut noise = NoiseMap::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let point: f64 = get_point(
                Coordinate::new(x as i32, y as i32),
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
    point: Coordinate,
    noises: &Noises,
    width: usize,
    height: usize,
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
    is_seamless: bool,
) -> Points {
    let x_extent = x2 - x1;
    let y_extent = y2 - y1;

    let x_step = x_extent / width as f64;
    let y_step = y_extent / height as f64;

    let current_x = x1 + x_step * point.x as f64;
    let current_y = y1 + y_step * point.y as f64;

    let points = get_points([current_x, current_y], noises);

    let final_value = if is_seamless {
        let sw_value = noises.height.get([current_x, current_y]);
        let se_value = noises.height.get([current_x + x_extent, current_y]);
        let nw_value = noises.height.get([current_x, current_y + y_extent]);
        let ne_value = noises
            .height
            .get([current_x + x_extent, current_y + y_extent]);

        let x_blend = 1.0 - ((current_x - x1) / x_extent);
        let y_blend = 1.0 - ((current_y - y1) / y_extent);

        let y0 = linear(sw_value, se_value, x_blend);
        let y1 = linear(nw_value, ne_value, x_blend);

        linear(y0, y1, y_blend)
    } else {
        noises.height.get([current_x, current_y])
    };

    Points::new(final_value, points.temperature, points.humidity)
}

pub fn get_point(
    point: Coordinate,
    noises: &Noises,
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
pub fn get_points(point: [f64; 2], noises: &Noises) -> Points {
    Points::new(
        noises.height.get(point),
        noises.temperature.get(point),
        noises.humidity.get(point),
    )
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

pub fn get_biome(points: Points) -> String {
    let biome: String;

    if points.height <= -0.3 {
        biome = "deep_ocean".to_string();
    } else if points.height <= -0.1 {
        biome = "ocean".to_string();
    } else if points.height <= 0.0 {
        biome = "sand".to_string();
    } else if points.height <= 0.5 {
        if points.temperature <= 0.0 {
            biome = "tundra".to_string();
        } else if points.temperature >= 0.7 {
            if points.humidity <= 0.0 {
                biome = "desert".to_string();
            } else if points.humidity <= 0.2 {
                biome = "grassland".to_string();
            } else if points.humidity <= 0.4 {
                biome = "forest".to_string();
            } else {
                biome = "rainforest".to_string();
            }
        } else if points.humidity <= 0.33 {
            biome = "grassland".to_string();
        } else if points.humidity >= 0.66 {
            biome = "forest".to_string();
        } else if points.height <= 0.2 {
            biome = "grassland".to_string();
        } else {
            biome = "forest".to_string();
        }
    } else if points.height <= 0.8 {
        biome = "mountain".to_string();
    } else {
        biome = "snow".to_string();
    }

    biome
}

fn build(chunk: &Chunk, noises: &Noises) -> NoiseMap {
    let mut result_map = NoiseMap::new(chunk.width, chunk.height);

    let noise = &noises.height;

    let x_bounds = (chunk.x1, chunk.x2);
    let y_bounds = (chunk.y1, chunk.y2);

    let x_extent = x_bounds.1 - x_bounds.0;
    let y_extent = y_bounds.1 - y_bounds.0;

    let x_step = x_extent / chunk.width as f64;
    let y_step = y_extent / chunk.height as f64;

    for y in 0..chunk.height {
        let current_y = y_bounds.0 + y_step * y as f64;

        for x in 0..chunk.width {
            let current_x = x_bounds.0 + x_step * x as f64;

            let points = get_points([current_x, current_y], &noises);

            let final_value = if chunk.is_seamless {
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
                points.height
            };

            result_map[(x, y)] = get_biome_as_f64(get_biome(Points::new(
                final_value,
                points.temperature,
                points.humidity,
            )));
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
