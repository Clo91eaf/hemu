#![allow(dead_code)]

// memory
pub const MEM_SIZE: u64 = 0x8000000;
pub const MEM_BASE: u64 = 0x80000000;
pub const PC_RESET_OFFSET: u64 = 0x0;
pub const MEM_LEFT: u64 = MEM_BASE;
pub const MEM_RIGHT: u64 = MEM_BASE + MEM_SIZE - 1;
pub const RESET_VECTOR: u64 = MEM_LEFT + PC_RESET_OFFSET;

