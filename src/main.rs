mod monitor;
mod engine;

use engine::init::engine_start;

fn main() {
    engine_start();
    println!("Hello, world!");
}
