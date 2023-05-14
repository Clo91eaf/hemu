mod engine;
mod monitor;
mod debug;
mod cpu;
mod memory;

use engine::init::engine_start;

fn main() {
    engine_start();
}
