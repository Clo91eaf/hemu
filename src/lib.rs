//! RISC-V emulator core implementation.
//!
//! # How to use
//! Create an `Emulator` object, place a binary data in DRAM and set the program counter to
//! `DRAM_BASE`. The binary data must contain no headers for now. The example is here:
/*
//! ```rust
//! use hemu::bus::DRAM_BASE;
//! use hemu::emulator::Emulator;
//!
//! fn main() {
//!     // Create a dummy binary data.
//!     let data = vec![
//!         0x93, 0x0f, 0xa0, 0x02, // addi x31, x0, 42
//!     ];
//!
//!     // Create an emulator object.
//!     let mut emu = Emulator::new(false, false);
//!     // Place the binary data in the beginning of DRAM.
//!     emu.initialize_dram(data);
//!     // Set the program counter to 0x8000_0000, which is the address DRAM starts.
//!     emu.initialize_pc(DRAM_BASE);
//!     // Start the emulator.
//!     emu.start();
//!
//!     // `IllegalInstruction` is raised for now because of the termination condition of the emulator,
//!     // but the register is successfully updated.
//!     assert_eq!(42, emu.cpu.gpr.read(31));
//! }
//! ```
*/

pub mod bus;
pub mod cpu;
pub mod devices;
pub mod dut;
pub mod emulator;
pub mod exception;
pub mod instructions;
pub mod interrupt;
pub mod log;
pub mod rom;
pub mod tui;
