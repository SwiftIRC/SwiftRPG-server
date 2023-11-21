mod map;

use map::generate_map;
use map::get_point;

fn main() {
    const CURRENT_SEED: u32 = 1000;

    let (_, noises) = generate_map(CURRENT_SEED);

    println!("[0, 0] => {}", get_point([-50.0625, -50.0625], &noises));
}
