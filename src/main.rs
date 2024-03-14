use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::path::PathBuf;

use hemu::bus::DRAM_BASE;
use hemu::cpu::Cpu;
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

  /// Enables to output debug messages
  #[arg(short = 'd', long = "debug")]
  debug: bool,

  /// Enables to count each instruction executed
  #[clap(short = 'c', long = "count")]
  count: bool,
}

/// Output current registers to the console.
fn dump_registers(cpu: &Cpu) {
  println!("-------------------------------------------------------------------------------------------");
  println!("{}", cpu.xregs);
  println!("-------------------------------------------------------------------------------------------");
  println!("{}", cpu.fregs);
  println!("-------------------------------------------------------------------------------------------");
  println!("{}", cpu.state);
  println!("-------------------------------------------------------------------------------------------");
  println!("pc: {:#x}", cpu.pc);
}

/// Output the count of each instruction executed.
fn dump_count(cpu: &Cpu) {
  if cpu.is_count {
    println!("===========================================================================================");
    let mut sorted_counter = Vec::from_iter(&cpu.inst_counter);
    sorted_counter.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    for (inst, count) in sorted_counter.iter() {
      println!("{}, {}", inst, count);
    }
    println!("===========================================================================================");
  }
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

  emu.is_debug = args.debug;

  emu.cpu.is_count = args.count;

  emu.start();

  dump_registers(&emu.cpu);
  dump_count(&emu.cpu);

  Ok(())
}
