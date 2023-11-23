extern crate rocket;

pub mod state;

use crate::map;
use state::RocketState;

use map::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rocket::{routes, State};
use serde_json::json;

#[get("/map/<x>/<y>")]
fn map_endpoint(x: i32, y: i32, state: &State<RocketState>) -> String {
    let point = (x, y);

    let mut r = StdRng::seed_from_u64(state.seed as u64);

    let converted_point = convert_point(
        [x as f64, y as f64],
        &state.noises,
        state.width,
        state.height,
        state.x1,
        state.x2,
        state.y1,
        state.y2,
        state.is_seamless,
    );

    let biome = get_biome(converted_point.to_owned());
    let height = get_biome_as_f64(biome.to_owned());

    let trees: i32;
    if vec!["forest", "grasslands", "rainforest", "mountain"].contains(&biome.as_str()) {
        trees = r.gen_range(r.to_owned().gen_range(0..10)..r.to_owned().gen_range(10..15));
    } else {
        trees = 0;
    }

    let point = Point::new(point, converted_point, height, biome, trees, 0);

    json!(point).to_string()
}

pub async fn rocket(state: RocketState) {
    match rocket::build()
        .manage(state)
        .mount("/api", routes![map_endpoint])
        .launch()
        .await
    {
        Ok(_) => println!("Rocket has shut down."),
        Err(e) => println!("Rocket has crashed: {}", e),
    };
}
