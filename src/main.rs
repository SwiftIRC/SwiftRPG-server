mod database;
mod game;
mod map;
mod web;

use ctrlc;
use database::*;
use map::data::Chunk;
use map::fns::generate_map;
use map::fns::generate_noises;
use std::thread;
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

    if std::env::var("SWIFTRPG_GENERATE_MAP").unwrap_or("false".to_string()) == "true" {
        thread::spawn(move || {
            generate_map(&chunk, &noises.to_owned());
        });
    }

    if std::env::var("SWIFTRPG_CORE").unwrap_or("false".to_string()) == "true" {
        let mut next_timestamp = 0;
        let mut next_tick = chrono::Local::now().timestamp();
        thread::spawn(move || loop {
            let now = chrono::Local::now();
            let tick = now.timestamp();

            #[allow(unused_mut)]
            let mut connection = establish_connection();

            if tick >= next_timestamp {
                next_timestamp = tick + 1;

                if next_tick <= tick {
                    let timestamp = now.format("%D %T").to_string();
                    println!("{}", timestamp);

                    game::tick(connection);

                    next_tick = tick + 60 - (next_tick % 60);
                }
            }
        });
    }

    if std::env::var("SWIFTRPG_WEBSERVER").unwrap_or("false".to_string()) == "true" {
        web::rocket(state.to_owned()).await;
    } else {
        // wait for ctrl+c
        let (tx, rx) = std::sync::mpsc::channel();
        ctrlc::set_handler(move || {
            tx.send(()).unwrap();
        })
        .expect("Error setting Ctrl-C handler");

        rx.recv().unwrap();
    }
}
