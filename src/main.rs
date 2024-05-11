use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use hemu::bus::DRAM_BASE;
use hemu::emulator::Emulator;
use hemu::log::log_trace;
use hemu::tui;

use clap::Parser;

/// Command line arguments.
#[derive(Parser, Debug)]
#[clap(name = "hemu: RISC-V emulator", version = "0.0.1", author = "Clo91eaf <@qq.com>")]
struct Args {
  /// A kernel ELF image without headers
  #[arg(short = 'k', long = "kernel", required = true)]
  kernel: PathBuf,

  /// A raw disk image
  #[arg(short = 'f', long = "file")]
  file: Option<PathBuf>,

  /// specify the log level. Including "error", "warn", "info", "debug", "trace"
  #[arg(short = 'l', long = "log", default_value = "info")]
  log: String,

  /// Enable difftest
  #[arg(short, long)]
  diff: bool,

  /// Enable tui
  #[arg(long)]
  tui: bool,

  /// Enable wave trace
  #[arg(long)]
  trace: bool,
}

/// Main function of RISC-V emulator for the CLI version.
fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  // Set the log level.
  // "error", "warn", "info", "debug", "trace"
  let level = args.log.parse()?;
  log_trace(level);

  assert!(!(args.tui && level < tracing::Level::INFO), "Tui requires log level >= INFO.");
  assert!(!(!args.diff && args.tui), "Tui without difftest is not supported yet.");

  // Read the kernel bin(after objcopy) and the disk image.
  let mut kernel_file = File::open(args.kernel)?;
  let mut kernel_data = Vec::new();
  kernel_file.read_to_end(&mut kernel_data)?;

  let mut img_data = Vec::new();
  if let Some(img_file) = args.file {
    File::open(img_file)?.read_to_end(&mut img_data)?;
  }

  // Create an emulator object and start the execution.
  let mut emu = Emulator::new(args.trace, args.diff);

  emu.reset();

  emu.initialize_dram(kernel_data);
  emu.initialize_disk(img_data);
  emu.initialize_pc(DRAM_BASE);

  match (args.diff, args.tui) {
    (true, true) => {
      let mut terminal = tui::init()?;
      emu.start_diff_tui(&mut terminal);
      tui::restore()?;
    }
    (true, false) => emu.start_diff(),
    (false, false) => emu.start(),
    _ => panic!("Tui without difftest is not supported yet."),
  }

  Ok(())
}
