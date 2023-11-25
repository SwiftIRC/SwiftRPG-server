extern crate rocket;

pub mod state;

use crate::map;
use state::RocketState;

use map::data::*;
use map::fns::*;
use map::resources::*;
use rocket::{routes, State};
use serde_json::json;

#[get("/map/<x>/<y>")]
fn map_endpoint(x: i32, y: i32, state: &State<RocketState>) -> String {
    let point = (x, y);

    let converted_point = convert_point(
        [x as f64, y as f64],
        &state.noises,
        state.chunk.width,
        state.chunk.height,
        state.chunk.x1,
        state.chunk.x2,
        state.chunk.y1,
        state.chunk.y2,
        state.chunk.is_seamless,
    );

    let biome = get_biome(converted_point.to_owned());
    let height = get_biome_as_f64(biome.to_owned());

    let resources = Resources::new(&state.noises, [x as f64, y as f64], &biome);

    let point = Point::new(point, converted_point, height, biome, resources);

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
