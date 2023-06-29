mod engine;
mod monitor;
mod debug;
mod cpu;
mod memory;
mod constants;

use engine::init::engine_start;

fn main() {
    engine_start();
}
