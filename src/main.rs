extern crate noise;

use noise::*;

fn main() {
    generate_map();
}

struct NoiseTemplate {
    name: String,
    frequency: f64,
    persistence: f64,
    lacunarity: f64,
    octaves: usize,
}

struct Noise {
    noise: Fbm<Perlin>,
    seed: u32,
    name: String,
}

fn get_map_types() -> Vec<NoiseTemplate> {
    vec![
        NoiseTemplate {
            name: "height".to_string(),
            frequency: 1.0,
            persistence: 0.5,
            lacunarity: 2.208984375,
            octaves: 14,
        },
        NoiseTemplate {
            name: "temperature".to_string(),
            frequency: 1.0,
            persistence: 0.5,
            lacunarity: 1.8,
            octaves: 14,
        },
        NoiseTemplate {
            name: "humidity".to_string(),
            frequency: 1.0,
            persistence: 0.5,
            lacunarity: 2.1,
            octaves: 14,
        },
    ]
}
fn generate_map() {
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
            seed: CURRENT_SEED + i as u32,
            name: t.name.to_owned(),
        });

        println!(
            "{}: {}",
            noises[i].name,
            noises[i].noise.get([100.5, 100.5])
        )
    });
}
