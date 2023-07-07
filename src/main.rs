mod engine;
mod monitor;
mod log;
mod cpu;
mod memory;
mod constants;

use engine::init::engine_start;

fn main() {
    engine_start();
}
