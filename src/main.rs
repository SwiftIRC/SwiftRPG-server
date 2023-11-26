extern crate rocket;

mod map;
mod web;

use map::data::Chunk;
use map::fns::generate_map;
use map::fns::generate_noises;
use std::thread;
use std::time::Duration;
use web::state::RocketState;

const CURRENT_SEED: u32 = 1000;
const CHUNK_WIDTH: usize = 160;
const CHUNK_HEIGHT: usize = 160;
const CHUNK_X1: f64 = 0.0;
const CHUNK_X2: f64 = 10.0;
const CHUNK_Y1: f64 = 0.0;
const CHUNK_Y2: f64 = 10.0;
const IS_SEAMLESS: bool = true;

#[rocket::main]
async fn main() {
    let chunk = Chunk::new(
        CHUNK_WIDTH,
        CHUNK_HEIGHT,
        CHUNK_X1,
        CHUNK_X2,
        CHUNK_Y1,
        CHUNK_Y2,
        IS_SEAMLESS,
    );

    let noises = generate_noises(CURRENT_SEED);

    let state = RocketState::new(noises.to_owned(), CURRENT_SEED, chunk.to_owned());

    thread::spawn(move || {
        generate_map(&chunk, &noises.to_owned());
    });

    thread::spawn(|| loop {
        let now = chrono::Local::now();
        let timestamp = now.format("%D %T").to_string();
        println!("{}", timestamp);

        thread::sleep(Duration::from_secs(5));
    });

    web::rocket(state.to_owned()).await;
}
