use std::env;
use std::path::Path;
use config::{Config, File};

fn main() {
  // sanity check
  let nemu_home = env::var("NEMU_HOME").unwrap_or_else(|_| "".to_string());
  let nemu_main_path = Path::new(&nemu_home).join("src/nemu-main.c");
  if !nemu_main_path.exists() {
    panic!("NEMU_HOME={} is not a NEMU repo", nemu_home);
  }

  // add config
  let cfg = Config::builder()
    .add_source(File::with_name("config.toml"));
  let guest_isa = cfg.get::<String>("ISA").unwrap();

  
  // todo: add more configs to trans Makefile to rust
}
