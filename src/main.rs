mod map;

use map::generate_map;
use map::reconstruct_map;

fn main() {
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

    reconstruct_map(
        CHUNK_WIDTH,
        CHUNK_HEIGHT,
        CHUNK_X1,
        CHUNK_X2,
        CHUNK_Y1,
        CHUNK_Y2,
        IS_SEAMLESS,
        &noises,
    );
}
