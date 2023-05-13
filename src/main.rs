mod engine;
mod monitor;
mod debug;
mod cpu;

use engine::init::engine_start;

fn main() {
    engine_start();
}
