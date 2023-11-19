mod map;

use map::generate_map;

fn main() {
    const CURRENT_SEED: u32 = 1000;

    generate_map(CURRENT_SEED);
}
