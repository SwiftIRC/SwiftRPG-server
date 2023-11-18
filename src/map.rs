extern crate noise;

use noise::{utils::*, *};

struct NoiseTemplate {
    name: String,
    frequency: f64,
    persistence: f64,
    lacunarity: f64,
    octaves: usize,
}

pub struct Noise {
    noise: Fbm<Perlin>,
    name: String,
}

fn get_map_types() -> Vec<NoiseTemplate> {
    vec![
        NoiseTemplate {
            name: "height".to_string(),
            frequency: 1.0,
            persistence: 0.5,
            lacunarity: 1.608984375,
            octaves: 4,
        },
        NoiseTemplate {
            name: "temperature".to_string(),
            frequency: 1.0,
            persistence: 0.5,
            lacunarity: 1.8,
            octaves: 4,
        },
        NoiseTemplate {
            name: "humidity".to_string(),
            frequency: 1.0,
            persistence: 0.5,
            lacunarity: 2.1,
            octaves: 4,
        },
    ]
}

pub fn generate_map() -> Vec<Noise> {
    const CURRENT_SEED: u32 = 10000;

    let types = get_map_types();
    let mut noises = Vec::new();

    types.iter().for_each(|t| {
        let i = types
            .iter()
            .position(|x| x.name == t.name)
            .unwrap_or_default();

        noises.push(Noise {
            noise: Fbm::<Perlin>::new(CURRENT_SEED + i as u32)
                .set_frequency(t.frequency)
                .set_persistence(t.persistence)
                .set_lacunarity(t.lacunarity)
                .set_octaves(t.octaves),
            name: t.name.to_owned(),
        });

        println!(
            "{}: {}",
            noises[i].name,
            noises[i].noise.get([100.5, 100.5])
        );

        imagerender_mapbuilder(&noises[i].noise, &format!("{}.png", noises[i].name));
    });

    print!("{:?}: ", get_point([100.5, 100.5], "10000"));

    noises
}

fn imagerender_mapbuilder(noise_map: &noise::Fbm<noise::Perlin>, filename: &str) {
    ImageRenderer::new()
        .render(&mapbuilder(&noise_map))
        .write_to_file(filename);
}

fn mapbuilder(noise_map: &noise::Fbm<noise::Perlin>) -> NoiseMap {
    PlaneMapBuilder::<&noise::Cache<&noise::Fbm<noise::Perlin>>, 2>::new(&Cache::new(noise_map))
        .set_size(16, 16)
        .set_x_bounds(-1.0, 1.0)
        .set_y_bounds(-1.0, 1.0)
        .build()
}

fn get_point(point: [f64; 2], seed: &str) -> Vec<f64> {
    let types = get_map_types();

    let mut points = Vec::new();

    types.iter().for_each(|t| {
        let i = types
            .iter()
            .position(|x| x.name == t.name)
            .unwrap_or_default();

        let noise = Fbm::<Perlin>::new(seed.parse::<u32>().unwrap() + i as u32)
            .set_frequency(t.frequency)
            .set_persistence(t.persistence)
            .set_lacunarity(t.lacunarity)
            .set_octaves(t.octaves);

        points.push(noise.get(point));

        println!("{}: {}", t.name, noise.get(point));
    });

    points
}

fn get_biome(point: [f64; 2], seed: &str) -> String {
    let points = get_point(point, seed);

    let height = points[0];
    let temperature = points[1];
    let humidity = points[2];

    let mut biome = String::new();

    if height < 0.0 {
        biome = "ocean".to_string();
    } else if height < 0.1 {
        biome = "beach".to_string();
    } else if height > 0.8 {
        biome = "snow".to_string();
    } else if height > 0.6 {
        biome = "tundra".to_string();
    } else if height > 0.1 {
        if temperature < 0.25 {
            biome = "bare".to_string();
        } else if temperature < 0.6 {
            biome = "taiga".to_string();
        } else {
            biome = "shrubland".to_string();
        }
    } else {
        if temperature < 0.25 {
            if humidity < 0.33 {
                biome = "scorched".to_string();
            } else if humidity < 0.66 {
                biome = "temperate desert".to_string();
            } else {
                biome = "desert".to_string();
            }
        } else if temperature < 0.6 {
            if humidity < 0.16 {
                biome = "tundra".to_string();
            } else if humidity < 0.50 {
                biome = "grassland".to_string();
            } else if humidity < 0.83 {
                biome = "temperate deciduous forest".to_string();
            } else {
                biome = "temperate rainforest".to_string();
            }
        } else {
            if humidity < 0.16 {
                biome = "shrubland".to_string();
            } else if humidity < 0.33 {
                biome = "taiga".to_string();
            } else if humidity < 0.66 {
                biome = "temperate rainforest".to_string();
            } else {
                biome = "tropical rainforest".to_string();
            }
        }
    }

    biome
}
