#[macro_use]
extern crate rocket;

mod map;
mod web;

use map::*;
use std::thread;
use std::time::Duration;
use web::state::RocketState;

#[rocket::main]
async fn main() {
    const CURRENT_SEED: u32 = 1000;

    const CHUNK_WIDTH: usize = 160;
    const CHUNK_HEIGHT: usize = 160;
    const CHUNK_X1: f64 = 0.0;
    const CHUNK_X2: f64 = 10.0;
    const CHUNK_Y1: f64 = 0.0;
    const CHUNK_Y2: f64 = 10.0;
    const IS_SEAMLESS: bool = true;

    let (_, noises) = generate_map(
        CHUNK_WIDTH,
        CHUNK_HEIGHT,
        CHUNK_X1,
        CHUNK_X2,
        CHUNK_Y1,
        CHUNK_Y2,
        IS_SEAMLESS,
        CURRENT_SEED,
    );

    thread::spawn(|| loop {
        let now = chrono::Local::now();
        let timestamp = now.format("%T").to_string();
        println!("{}", timestamp);

        thread::sleep(Duration::from_secs(5));
    });

    let state = RocketState::new(
        noises,
        CURRENT_SEED,
        CHUNK_WIDTH,
        CHUNK_HEIGHT,
        CHUNK_X1,
        CHUNK_X2,
        CHUNK_Y1,
        CHUNK_Y2,
        IS_SEAMLESS,
    );

    web::rocket(state.to_owned()).await;
}
