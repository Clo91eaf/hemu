use crate::cpu::Cpu;
use crate::monitor::{init_monitor, sdb};
use clap::Parser;
use std::path::PathBuf;

/// A riscv64 monitor write in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Run in batch mode
  #[arg(short, long, default_value = "false")]
  batch: bool,

  /// Diff file
  #[arg(short, long, default_value = "resources/am-tests/add-riscv64-nemu.diff")]
  diff: PathBuf,

  /// Img file
  #[arg(short = 'f', long, default_value = "resources/am-tests/add-riscv64-nemu.bin")]
  img: PathBuf,
}

pub fn engine_start() {
  let args = Args::parse();

  let _ = init_monitor(args.img, "info".to_string());

  let cpu = &mut Cpu::new(args.diff);

  sdb::sdb_mainloop(cpu, args.batch);
}
