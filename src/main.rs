use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use hemu::bus::DRAM_BASE;
use hemu::emulator::Emulator;

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
}

/// Main function of RISC-V emulator for the CLI version.
fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  // Read the kernel bin(after objcopy) and the disk image.
  let mut kernel_file = File::open(args.kernel)?;
  let mut kernel_data = Vec::new();
  kernel_file.read_to_end(&mut kernel_data)?;

  let mut img_data = Vec::new();
  if let Some(img_file) = args.file {
    File::open(img_file)?.read_to_end(&mut img_data)?;
  }

  // Create an emulator object and start the execution.
  let mut emu = Emulator::new();

  emu.initialize_dram(kernel_data);
  emu.initialize_disk(img_data);
  emu.initialize_pc(DRAM_BASE);

  emu.start();

  Ok(())
}
