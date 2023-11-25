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

    let tree_count: i32 = trees(&state.noises, [x as f64, y as f64], &biome);
    let rock_count: i32 = rocks(&state.noises, [x as f64, y as f64], &biome);
    let herb_count: i32 = herbs(&state.noises, [x as f64, y as f64], &biome);
    let fruit_count: i32 = fruits(&state.noises, [x as f64, y as f64], &biome);

    let resources = Resources {
        trees: tree_count,
        rocks: rock_count,
        herbs: herb_count,
        fruits: fruit_count,
    };

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
